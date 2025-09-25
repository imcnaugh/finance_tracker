use crate::model::account::Account;
use clap::Args;

#[derive(Args, Debug)]
pub struct NewTransaction {
    debit_account: Account,
    credit_account: Account,
    amount: i64,
    description: String,
}

impl NewTransaction {
    pub fn get_debit_account(&self) -> &Account {
        &self.debit_account
    }

    pub fn get_credit_account(&self) -> &Account {
        &self.credit_account
    }

    pub fn get_amount(&self) -> i64 {
        self.amount
    }

    pub fn get_description(&self) -> &String {
        &self.description
    }
}
