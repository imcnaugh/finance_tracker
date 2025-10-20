use double_entry_bookkeeping::dao::account_dao::AccountDao;
use double_entry_bookkeeping::model::{Account, AccountType, NewAccount, NewAccountType};
use sqlx::{Error, Executor, Pool, Sqlite};

pub struct AccountSqliteDao {
    pool: Pool<Sqlite>,
}

const INSERT_ACCOUNT_TYPE_SQL: &str = r#"
INSERT INTO account_type (
    name,
    normal_balance
) VALUES (?, ?)
"#;

const SELECT_ALL_ACCOUNT_TYPE_SQL: &str = r#"
SELECT
    id,
    name,
    normal_balance,
    created_timestamp
FROM account_type
"#;

const SELECT_ACCOUNT_TYPE_BY_ID_SQL: &str = r#"
SELECT
    id,
    name,
    normal_balance,
    created_timestamp
FROM account_type
WHERE id = ?
"#;

const INSERT_ACCOUNT_SQL: &str = r#"
INSERT INTO account (
    account_type_id,
    name
) VALUES (?, ?)
"#;

const SELECT_ACCOUNT_BY_ID_SQL: &str = r#"
SELECT
    id,
    account_type_id,
    name,
    created_timestamp
FROM account
WHERE id = ?
"#;

const SELECT_ALL_ACCOUNT_SQL: &str = r#"
SELECT
    id,
    account_type_id,
    name,
    created_timestamp
FROM account
"#;

impl AccountSqliteDao {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    async fn insert_account_type<'e, E>(
        &self,
        executor: E,
        new_account_type: NewAccountType,
    ) -> Result<u64, Error>
    where
        E: Executor<'e, Database = Sqlite>,
    {
        let query = sqlx::query(INSERT_ACCOUNT_TYPE_SQL)
            .bind(new_account_type.get_name())
            .bind(new_account_type.get_normal_balance());

        let result = query.execute(executor).await?;
        Ok(result.last_insert_rowid() as u64)
    }

    async fn read_all_account_type<'e, E>(&self, executor: E) -> Result<Vec<AccountType>, Error>
    where
        E: Executor<'e, Database = Sqlite>,
    {
        let query = sqlx::query_as::<_, AccountType>(SELECT_ALL_ACCOUNT_TYPE_SQL);
        query.fetch_all(executor).await
    }

    async fn read_account_type_by_id<'e, E>(
        &self,
        account_type_id: u64,
        executor: E,
    ) -> Result<Option<AccountType>, Error>
    where
        E: Executor<'e, Database = Sqlite>,
    {
        let item = sqlx::query_as::<_, AccountType>(SELECT_ACCOUNT_TYPE_BY_ID_SQL)
            .bind(account_type_id as i32)
            .fetch_optional(executor)
            .await?;

        Ok(item)
    }

    async fn insert_account<'e, E>(
        &self,
        executor: E,
        new_account: NewAccount,
    ) -> Result<u64, Error>
    where
        E: Executor<'e, Database = Sqlite>,
    {
        let query = sqlx::query(INSERT_ACCOUNT_SQL)
            .bind(new_account.get_account_type_id() as i32)
            .bind(new_account.get_name());

        let result = query.execute(executor).await?;
        Ok(result.last_insert_rowid() as u64)
    }

    async fn read_account_by_id<'e, E>(
        &self,
        executor: E,
        account_id: u64,
    ) -> Result<Option<Account>, Error>
    where
        E: Executor<'e, Database = Sqlite>,
    {
        let item = sqlx::query_as::<_, Account>(SELECT_ACCOUNT_BY_ID_SQL)
            .bind(account_id as i32)
            .fetch_optional(executor)
            .await?;

        Ok(item)
    }

    async fn read_all_account<'e, E>(&self, executor: E) -> Result<Vec<Account>, Error>
    where
        E: Executor<'e, Database = Sqlite>,
    {
        let item = sqlx::query_as::<_, Account>(SELECT_ALL_ACCOUNT_SQL)
            .fetch_all(executor)
            .await?;
        Ok(item)
    }
}

impl AccountDao for AccountSqliteDao {
    async fn create_account_type(&self, new_account_type: NewAccountType) -> Result<u64, Error> {
        let mut conn = self.pool.acquire().await?;
        let account_type_id = self
            .insert_account_type(&mut *conn, new_account_type)
            .await?;
        Ok(account_type_id)
    }

    async fn get_account_type_by_id(
        &self,
        account_type_id: u64,
    ) -> Result<Option<AccountType>, Error> {
        let mut conn = self.pool.acquire().await?;
        let account_type = self
            .read_account_type_by_id(account_type_id, &mut *conn)
            .await?;
        Ok(account_type)
    }

    async fn get_all_account_types(&self) -> Result<Vec<AccountType>, Error> {
        let mut conn = self.pool.acquire().await?;
        let account_types = self.read_all_account_type(&mut *conn).await?;
        Ok(account_types)
    }

    async fn create_account(&self, new_account: NewAccount) -> Result<u64, Error> {
        let mut conn = self.pool.acquire().await?;
        let account_id = self.insert_account(&mut *conn, new_account).await?;
        Ok(account_id)
    }

    async fn get_account_by_id(&self, account_id: u64) -> Result<Option<Account>, Error> {
        let mut conn = self.pool.acquire().await?;
        let account = self.read_account_by_id(&mut *conn, account_id).await?;
        Ok(account)
    }

    async fn get_all_accounts(&self) -> Result<Vec<Account>, Error> {
        let mut conn = self.pool.acquire().await?;
        let accounts = self.read_all_account(&mut *conn).await?;
        Ok(accounts)
    }
}
