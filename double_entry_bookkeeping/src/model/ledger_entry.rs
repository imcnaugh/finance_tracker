use crate::model::new_transaction::NewTransaction;
use crate::model::journal::Journal;
use crate::model::ledger::Ledger;
use chrono::Utc;

pub struct LedgerEntry;

impl LedgerEntry {
    /// Creates a journal entry and two ledger entries (debit and credit) from a transaction
    pub fn from_transaction(new_transaction: &NewTransaction) -> (Journal, Ledger, Ledger) {
        let current_timestamp = Utc::now().timestamp();

        // Create the journal entry (we'll use 0 as placeholder ID since it's auto-increment)
        let journal = Journal::new(
            0, // Will be set by database
            new_transaction.get_description().to_string(),
            current_timestamp,
        );

        // Create debit ledger entry
        let debit_ledger = Ledger::new(
            0, // Will be set by database
            new_transaction.get_debit_account_id(),
            0, // Will be set to journal ID after journal is created
            new_transaction.get_amount_in_cents(),
            true, // is_debit = true
            current_timestamp,
        );

        // Create credit ledger entry
        let credit_ledger = Ledger::new(
            0, // Will be set by database
            new_transaction.get_credit_account_id(),
            0, // Will be set to journal ID after journal is created
            new_transaction.get_amount_in_cents(),
            false, // is_debit = false
            current_timestamp,
        );

        (journal, debit_ledger, credit_ledger)
    }
}
