use crate::dao::account_dao::AccountDao;
use crate::dao::journal_dao::JournalDao;
use crate::model::{Account, AccountType};

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

    pub async fn get_all_accounts(&self) -> Result<Vec<Account>, String> {
        let accounts = self
            .account_dao
            .get_all_accounts()
            .await
            .map_err(|e| e.to_string())?;

        Ok(accounts)
    }

    pub async fn get_account_by_id(&self, account_id: u64) -> Result<Option<Account>, String> {
        let account = self
            .account_dao
            .get_account_by_id(account_id)
            .await
            .map_err(|e| e.to_string())?;

        Ok(account)
    }
}
