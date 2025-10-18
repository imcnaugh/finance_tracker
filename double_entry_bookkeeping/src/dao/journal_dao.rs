use crate::model::{JournalEntry, NewJournalEntry, Transaction};

pub trait JournalDao {
    fn create_journal_entry(
        &self,
        new_journal_entry: NewJournalEntry,
    ) -> impl Future<Output = Result<u64, sqlx::Error>>;

    fn get_journal_entry_by_id(
        &self,
        journal_entry_id: u64,
    ) -> impl Future<Output = Result<Option<JournalEntry>, sqlx::Error>>;

    fn get_transactions_by_journal_entry_id(
        &self,
        journal_entry_id: u64,
    ) -> impl Future<Output = Result<Vec<Transaction>, sqlx::Error>>;

    fn get_account_balance(
        &self,
        account_id: u64,
    ) -> impl Future<Output = Result<i64, sqlx::Error>>;
}
