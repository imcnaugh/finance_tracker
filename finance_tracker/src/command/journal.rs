use clap::Subcommand;
use double_entry_bookkeeping::model::NewJournalEntry;

#[derive(Subcommand)]
pub enum JournalSubCommands {
    #[command(visible_alias = "new")]
    NewTransaction {
        #[command(flatten)]
        new_journal_entry: NewJournalEntry,
    },
}
