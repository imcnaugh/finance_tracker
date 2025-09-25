use crate::model::account::Account;
use clap::Args;

#[derive(Args, Debug)]
pub struct NewTransaction {
    from_account: Account,
    to_account: Account,
    amount: i64,
    description: String,
}

impl NewTransaction {
    pub fn get_from_account(&self) -> &Account {
        &self.from_account
    }

    pub fn get_to_account(&self) -> &Account {
        &self.to_account
    }

    pub fn get_amount(&self) -> i64 {
        self.amount
    }

    pub fn get_description(&self) -> &String {
        &self.description
    }
}
