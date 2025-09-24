use crate::model::NewTransaction;

pub struct LedgerService {}

impl LedgerService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn make_transaction(&self, new_transaction: NewTransaction) {}
}
