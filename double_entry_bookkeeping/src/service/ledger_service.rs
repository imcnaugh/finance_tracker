use crate::model::LedgerEntry;
use crate::model::account::Account;

pub struct LedgerService {}

impl LedgerService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn make_transaction(
        &self,
        from_account: &Account,
        to_account: &Account,
        amount: i64,
        description: &str,
    ) {
        let from_transaction = LedgerEntry::new();
    }
}
