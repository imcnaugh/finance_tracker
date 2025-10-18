use double_entry_bookkeeping::dao::journal_dao::JournalDao;
use double_entry_bookkeeping::model::{JournalEntry, NewJournalEntry, Transaction};
use sqlx::{Error, Pool, Sqlite};

pub struct JournalSqliteDao {
    pool: Pool<Sqlite>,
}

impl JournalSqliteDao {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
}

impl JournalDao for JournalSqliteDao {
    async fn create_journal_entry(
        &self,
        new_journal_entry: NewJournalEntry,
    ) -> Result<JournalEntry, Error> {
        todo!()
    }

    async fn get_journal_entry_by_id(
        &self,
        journal_entry_id: u64,
    ) -> Result<Option<JournalEntry>, Error> {
        todo!()
    }

    async fn get_transactions_by_journal_entry_id(
        &self,
        journal_entry_id: u64,
    ) -> Result<Vec<Transaction>, Error> {
        todo!()
    }

    async fn get_account_balance(&self, account_id: u64) -> Result<i64, Error> {
        todo!()
    }
}
