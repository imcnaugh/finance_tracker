use double_entry_bookkeeping::dao::journal_dao::JournalDao;
use double_entry_bookkeeping::model::{JournalEntry, NewJournalEntry, Transaction};
use sqlx::{Connection, Error, Executor, Pool, Sqlite};

pub struct JournalSqliteDao {
    pool: Pool<Sqlite>,
}

const INSERT_JOURNAL_ENTRY_SQL: &str = r#"
INSERT INTO journal_entry (
    description
) VALUES
(?)
"#;

const INSERT_JOURNAL_TRANSACTION_SQL: &str = r#"
INSERT INTO journal_transaction (
    account_id,
    journal_entry_id,
    amount_in_cents,
    is_debit
) VALUES
(?, ?, ?, ?)
"#;

const SELECT_JOURNAL_ENTRY_SQL: &str = r#"
SELECT
    id,
    description,
    created_timestamp
FROM journal_entry
WHERE id = ?
"#;

const SELECT_JOURNAL_TRANSACTION_SQL: &str = r#"
SELECT
    id,
    account_id,
    journal_entry_id,
    amount_in_cents,
    is_debit,
    created_timestamp
FROM journal_transaction
WHERE journal_entry_id = ?
"#;

impl JournalSqliteDao {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    async fn insert_journal_entry<'e, E>(
        &self,
        executor: E,
        description: &str,
    ) -> Result<u64, Error>
    where
        E: Executor<'e, Database = Sqlite>,
    {
        let query = sqlx::query(INSERT_JOURNAL_ENTRY_SQL).bind(description);

        let result = query.execute(executor).await?;
        Ok(result.last_insert_rowid() as u64)
    }

    async fn insert_journal_transaction<'e, E>(
        &self,
        executor: E,
        account_id: u64,
        journal_entry_id: u64,
        amount_in_cents: i64,
        is_debit: bool,
    ) -> Result<u64, Error>
    where
        E: Executor<'e, Database = Sqlite>,
    {
        let query = sqlx::query(INSERT_JOURNAL_TRANSACTION_SQL)
            .bind(account_id as i32)
            .bind(journal_entry_id as i32)
            .bind(amount_in_cents)
            .bind(is_debit);

        let result = query.execute(executor).await?;

        Ok(result.last_insert_rowid() as u64)
    }

    async fn select_journal_entry<'e, E>(
        &self,
        executor: E,
        journal_entry_id: u64,
    ) -> Result<Option<JournalEntry>, Error>
    where
        E: Executor<'e, Database = Sqlite>,
    {
        let item = sqlx::query_as::<_, JournalEntry>(SELECT_JOURNAL_ENTRY_SQL)
            .bind(journal_entry_id as i32)
            .fetch_optional(executor)
            .await?;

        Ok(item)
    }

    async fn select_journal_transaction<'e, E>(
        &self,
        executor: E,
        journal_entry_id: u64,
    ) -> Result<Vec<Transaction>, Error>
    where
        E: Executor<'e, Database = Sqlite>,
    {
        let items = sqlx::query_as::<_, Transaction>(SELECT_JOURNAL_TRANSACTION_SQL)
            .bind(journal_entry_id as i32)
            .fetch_all(executor)
            .await?;

        Ok(items)
    }
}

impl JournalDao for JournalSqliteDao {
    async fn create_journal_entry(&self, new_journal_entry: NewJournalEntry) -> Result<u64, Error> {
        let mut conn = self.pool.acquire().await?;
        let mut tx = conn.begin().await?;
        let journal_entry_id = self
            .insert_journal_entry(&mut *tx, new_journal_entry.get_description())
            .await?;

        self.insert_journal_transaction(
            &mut *tx,
            new_journal_entry.get_debit_account_id(),
            journal_entry_id,
            new_journal_entry.get_amount_in_cents(),
            true,
        )
        .await?;

        self.insert_journal_transaction(
            &mut *tx,
            new_journal_entry.get_credit_account_id(),
            journal_entry_id,
            new_journal_entry.get_amount_in_cents(),
            false,
        )
        .await?;

        tx.commit().await?;

        Ok(journal_entry_id)
    }

    async fn get_journal_entry_by_id(
        &self,
        journal_entry_id: u64,
    ) -> Result<Option<JournalEntry>, Error> {
        let mut conn = self.pool.acquire().await?;
        let journal_entry = self
            .select_journal_entry(&mut *conn, journal_entry_id)
            .await?;
        Ok(journal_entry)
    }

    async fn get_transactions_by_journal_entry_id(
        &self,
        journal_entry_id: u64,
    ) -> Result<Vec<Transaction>, Error> {
        let mut conn = self.pool.acquire().await?;
        let transactions = self
            .select_journal_transaction(&mut *conn, journal_entry_id)
            .await?;
        Ok(transactions)
    }

    async fn get_account_balance(&self, account_id: u64) -> Result<i64, Error> {
        todo!()
    }
}
