use crate::command_handler::account::AccountCommandHandler;
use crate::command_handler::client::ClientCommandHandler;
use crate::command_handler::invoice::InvoiceCommandHandler;
use crate::command_handler::journal::JournalCommandHandler;
use crate::config_service::get_config;
use crate::configuration::Configuration;
use crate::database::DatabaseManager;
use double_entry_bookkeeping::account_service::AccountService;
use double_entry_bookkeeping::journal_service::JournalService;
use invoice_manager::client_service::ClientService;
use invoice_manager::invoice_service::InvoiceService;
use sqlite_dao::account_sqlite_dao::AccountSqliteDao;
use sqlite_dao::client_sqlite_dao::ClientSqliteDao;
use sqlite_dao::invoice_sqlite_dao::InvoiceSqliteDao;
use sqlite_dao::journal_sqlite_dao::JournalSqliteDao;
use std::sync::Arc;
use utilities::prompt_confirm;

pub struct Context {
    configs: Arc<Configuration>,
    client_service: Arc<ClientService<ClientSqliteDao>>,
    invoice_service: Arc<InvoiceService<InvoiceSqliteDao>>,
    account_service: Arc<AccountService<AccountSqliteDao>>,
    journal_service: Arc<JournalService<JournalSqliteDao>>,
}

impl Context {
    pub async fn try_build() -> Result<Self, String> {
        let configs = get_config().map(Arc::new)?;

        let db_manager = DatabaseManager::new(configs.get_database_configuration()).await?;

        let client_dao = Arc::new(ClientSqliteDao::new(db_manager.get_pool().clone()));
        let client_service = Arc::new(ClientService::new(client_dao.clone()));

        let invoice_dao = Arc::new(InvoiceSqliteDao::new(db_manager.get_pool().clone()));
        let invoice_service = Arc::new(InvoiceService::new(
            Some(prompt_confirm),
            invoice_dao.clone(),
        ));

        let account_dao = Arc::new(AccountSqliteDao::new(db_manager.get_pool().clone()));
        let account_service = Arc::new(AccountService::new(account_dao.clone()));

        let journal_dao = Arc::new(JournalSqliteDao::new(db_manager.get_pool().clone()));
        let journal_service = Arc::new(JournalService::new(journal_dao.clone()));

        Ok(Self {
            configs,
            client_service,
            invoice_service,
            account_service,
            journal_service,
        })
    }

    pub fn get_client_command_handler(&self) -> ClientCommandHandler {
        ClientCommandHandler::new(self.client_service.clone())
    }

    pub fn get_invoice_command_handler(&self) -> InvoiceCommandHandler {
        InvoiceCommandHandler::new(
            self.client_service.clone(),
            self.invoice_service.clone(),
            self.configs.clone(),
        )
    }

    pub fn get_account_command_handler(&self) -> AccountCommandHandler {
        AccountCommandHandler::new(self.account_service.clone())
    }

    pub fn get_journal_command_handler(&self) -> JournalCommandHandler {
        JournalCommandHandler::new(self.journal_service.clone())
    }
}
