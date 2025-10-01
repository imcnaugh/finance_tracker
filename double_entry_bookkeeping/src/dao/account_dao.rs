use crate::model::{Account, AccountType, NewAccount, NewAccountType};

pub trait AccountDao {
    fn create_account_type(
        &self,
        new_account_type: NewAccountType,
    ) -> impl Future<Output = Result<AccountType, sqlx::Error>>;

    fn get_account_type_by_id(
        &self,
        account_type_id: u64,
    ) -> impl Future<Output = Result<Option<AccountType>, sqlx::Error>>;

    fn get_all_account_types(&self) -> impl Future<Output = Result<Vec<AccountType>, sqlx::Error>>;

    fn create_account(
        &self,
        new_account: NewAccount,
    ) -> impl Future<Output = Result<Account, sqlx::Error>>;

    fn get_account_by_id(
        &self,
        account_id: u64,
    ) -> impl Future<Output = Result<Option<Account>, sqlx::Error>>;

    fn get_all_accounts(&self) -> impl Future<Output = Result<Vec<Account>, sqlx::Error>>;
}
