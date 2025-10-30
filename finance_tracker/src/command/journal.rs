use clap::{Args, Subcommand};
use double_entry_bookkeeping::model::NewJournalEntry;

#[derive(Subcommand)]
pub enum JournalSubCommands {
    #[command(visible_alias = "new")]
    NewTransaction {
        #[command(flatten)]
        new_journal_entry: SimpleJournalEntry,
    },
}

#[derive(Debug, Clone, Args)]
pub struct SimpleJournalEntry {
    description: String,
    amount: f64,
    debit_account_id: u64,
    credit_account_id: u64,
}

impl From<SimpleJournalEntry> for NewJournalEntry {
    fn from(value: SimpleJournalEntry) -> Self {
        let amount = (value.amount * 100.0) as i64;

        let transaction = vec![
            double_entry_bookkeeping::model::NewTransaction::new(
                value.debit_account_id,
                amount,
                true,
            ),
            double_entry_bookkeeping::model::NewTransaction::new(
                value.credit_account_id,
                amount,
                false,
            ),
        ];

        NewJournalEntry::new(value.description, transaction)
    }
}
