use crate::model::new_transaction::NewTransaction;
use chrono::Utc;
use utilities::utils::generate_new_id;

pub struct LedgerEntry {
    id: String,
    account_id: usize,
    timestamp: i64,
    description: String,
    debit_in_cents: i64,
    credit_in_cents: i64,
}

impl LedgerEntry {
    pub fn from(new_transaction: &NewTransaction) -> (Self, Self) {
        let current_timestamp = Utc::now().timestamp();
        let debit_entry = LedgerEntry {
            id: generate_new_id(),
            account: new_transaction.get_debit_account().clone(),
            timestamp: current_timestamp,
            description: new_transaction.get_description().to_string(),
            debit_in_cents: new_transaction.get_amount(),
            credit_in_cents: 0,
        };

        let credit_entry = LedgerEntry {
            id: generate_new_id(),
            account: new_transaction.get_credit_account().clone(),
            timestamp: current_timestamp,
            description: new_transaction.get_description().to_string(),
            debit_in_cents: 0,
            credit_in_cents: new_transaction.get_amount(),
        };

        (debit_entry, credit_entry)
    }
}
