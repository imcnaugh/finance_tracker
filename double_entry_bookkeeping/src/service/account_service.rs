use crate::dao::account_dao::AccountDao;
use crate::model::AccountType;

pub struct AccountService<A: AccountDao> {
    account_dao: A,
}

impl<A: AccountDao> AccountService<A> {
    pub fn new(account_dao: A) -> Self {
        Self { account_dao }
    }

    pub async fn get_all_account_types(&self) -> Result<Vec<AccountType>, String> {
        let account_types = self
            .account_dao
            .get_all_account_types()
            .await
            .map_err(|e| e.to_string())?;

        Ok(account_types)
    }
}
