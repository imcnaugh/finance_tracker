use crate::command::journal::JournalSubCommands;
use crate::command_handler::CommandHandler;
use crate::sqlite_dao::journal_sqlite_dao::JournalSqliteDao;
use double_entry_bookkeeping::journal_service::JournalService;
use double_entry_bookkeeping::model::NewJournalEntry;
use std::sync::Arc;

pub struct JournalCommandHandler {
    journal_service: Arc<JournalService<JournalSqliteDao>>,
}

impl JournalCommandHandler {
    pub fn new(journal_service: Arc<JournalService<JournalSqliteDao>>) -> Self {
        Self { journal_service }
    }
}

impl CommandHandler<JournalSubCommands> for JournalCommandHandler {
    async fn handle_command(&self, command: JournalSubCommands) {
        match command {
            JournalSubCommands::NewTransaction { new_journal_entry } => {
                match self
                    .journal_service
                    .make_transaction(NewJournalEntry::from(new_journal_entry))
                    .await
                {
                    Ok(journal_entry_id) => {
                        println!("Journal entry created with id: {}", journal_entry_id)
                    }
                    Err(_) => println!("Error creating journal entry"),
                }
            }
        }
    }
}
