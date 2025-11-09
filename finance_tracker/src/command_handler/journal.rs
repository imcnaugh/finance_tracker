use crate::command::journal::JournalSubCommands;
use crate::config_service::get_config;
use crate::configuration::Configuration;
use crate::database::DatabaseManager;
use crate::sqlite_dao::journal_sqlite_dao::JournalSqliteDao;
use double_entry_bookkeeping::model::NewJournalEntry;
use double_entry_bookkeeping::service::journal_service::JournalService;
use std::sync::Arc;

pub struct JournalCommandHandler {
    journal_service: Arc<JournalService<JournalSqliteDao>>,
    _configuration: Configuration,
}

impl JournalCommandHandler {
    pub fn new(
        journal_service: Arc<JournalService<JournalSqliteDao>>,
        configuration: Configuration,
    ) -> Self {
        Self {
            journal_service,
            _configuration: configuration,
        }
    }

    pub async fn handle_journal_command(&self, journal_command: JournalSubCommands) {
        match journal_command {
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
