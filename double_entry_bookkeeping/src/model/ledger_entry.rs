use crate::model::account::Account;
use crate::model::new_transaction::NewTransaction;

pub struct LedgerEntry {
    id: usize,
    account: Account,
    timestamp: i64,
    description: String,
    debit_in_cents: i64,
    credit_in_cents: i64,
}

impl LedgerEntry {
    pub fn from(new_transaction: NewTransaction) -> (Self, Self) {
        todo!()
    }
}
