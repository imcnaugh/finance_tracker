use crate::dao::account_dao::AccountDao;
use crate::model::{Account, AccountType, NewAccount, NewAccountType};
use sqlx::{Error, Executor, Pool, Sqlite};

pub struct AccountSqliteDao {
    pool: Pool<Sqlite>,
}

const INSERT_ACCOUNT_TYPE_SQL: &str = r#"
INSERT INTO account_type (
    name
) VALUES (?)
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
        let query = sqlx::query(INSERT_ACCOUNT_TYPE_SQL).bind(new_account_type.get_name());

        let result = query.execute(executor).await?;
        Ok(result.last_insert_rowid() as u64)
    }
}

impl AccountDao for AccountSqliteDao {
    async fn create_account_type(
        &self,
        new_account_type: NewAccountType,
    ) -> Result<AccountType, Error> {
        todo!()
    }

    async fn get_account_type_by_id(
        &self,
        account_type_id: u64,
    ) -> Result<Option<AccountType>, Error> {
        todo!()
    }

    async fn get_all_account_types(&self) -> Result<Vec<AccountType>, Error> {
        todo!()
    }

    async fn create_account(&self, new_account: NewAccount) -> Result<Account, Error> {
        todo!()
    }

    async fn get_account_by_id(&self, account_id: u64) -> Result<Option<Account>, Error> {
        todo!()
    }

    async fn get_all_accounts(&self) -> Result<Vec<Account>, Error> {
        todo!()
    }
}
