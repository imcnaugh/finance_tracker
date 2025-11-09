use crate::command_handler::account::AccountCommandHandler;
use crate::command_handler::client::ClientCommandHandler;
use crate::command_handler::invoice::InvoiceCommandHandler;
use crate::command_handler::journal::JournalCommandHandler;
use crate::config_service::get_config;
use crate::configuration::Configuration;
use crate::database::DatabaseManager;
use crate::sqlite_dao::account_sqlite_dao::AccountSqliteDao;
use crate::sqlite_dao::client_sqlite_dao::ClientSqliteDao;
use crate::sqlite_dao::invoice_sqlite_dao::InvoiceSqliteDao;
use crate::sqlite_dao::journal_sqlite_dao::JournalSqliteDao;
use double_entry_bookkeeping::service::account_service::AccountService;
use double_entry_bookkeeping::service::journal_service::JournalService;
use invoice_manager::service::{ClientService, InvoiceService};
use std::sync::Arc;
use utilities::prompt_confirm;

pub struct Context {
    configs: Result<Arc<Configuration>, String>,
    client_service: Result<Arc<ClientService<ClientSqliteDao>>, String>,
    invoice_service: Result<Arc<InvoiceService<InvoiceSqliteDao>>, String>,
    account_service: Result<Arc<AccountService<AccountSqliteDao>>, String>,
    journal_service: Result<Arc<JournalService<JournalSqliteDao>>, String>,
}

impl Context {
    pub async fn new() -> Self {
        let configs = get_config().map(Arc::new);

        let db_manager = Self::build_database_manager(&configs).await.map(Arc::new);

        let client_dao = Self::build_client_dao(&db_manager).map(Arc::new);
        let client_service = Self::build_client_service(client_dao).map(Arc::new);

        let invoice_dao = Self::build_invoice_dao(&db_manager).map(Arc::new);
        let invoice_service =
            Self::build_invoice_service(invoice_dao, Some(prompt_confirm)).map(Arc::new);

        let account_dao = Self::build_account_dao(&db_manager).map(Arc::new);
        let account_service = Self::build_account_service(account_dao).map(Arc::new);

        let journal_dao = Self::build_journal_dao(&db_manager).map(Arc::new);
        let journal_service = Self::build_journal_service(journal_dao).map(Arc::new);

        Self {
            configs,
            client_service,
            invoice_service,
            account_service,
            journal_service,
        }
    }

    pub fn get_client_command_handler(&self) -> Result<ClientCommandHandler, String> {
        Ok(ClientCommandHandler::new(
            self.client_service.as_ref()?.clone(),
        ))
    }

    pub fn get_invoice_command_handler(&self) -> Result<InvoiceCommandHandler, String> {
        Ok(InvoiceCommandHandler::new(
            self.client_service.as_ref()?.clone(),
            self.invoice_service.as_ref()?.clone(),
            self.configs.clone()?,
        ))
    }

    pub fn get_account_command_handler(&self) -> Result<AccountCommandHandler, String> {
        Ok(AccountCommandHandler::new(
            self.account_service.as_ref()?.clone(),
        ))
    }

    pub fn get_journal_command_handler(&self) -> Result<JournalCommandHandler, String> {
        Ok(JournalCommandHandler::new(
            self.journal_service.as_ref()?.clone(),
        ))
    }

    fn build_client_service(
        client_sqlite_dao: Result<Arc<ClientSqliteDao>, String>,
    ) -> Result<ClientService<ClientSqliteDao>, String> {
        Ok(ClientService::new(client_sqlite_dao.as_ref()?.clone()))
    }

    fn build_invoice_service(
        invoice_sqlite_dao: Result<Arc<InvoiceSqliteDao>, String>,
        confirm_fn: Option<fn(&str) -> bool>,
    ) -> Result<InvoiceService<InvoiceSqliteDao>, String> {
        Ok(InvoiceService::new(
            confirm_fn,
            invoice_sqlite_dao.as_ref()?.clone(),
        ))
    }

    fn build_account_service(
        account_sqlite_dao: Result<Arc<AccountSqliteDao>, String>,
    ) -> Result<AccountService<AccountSqliteDao>, String> {
        Ok(AccountService::new(account_sqlite_dao.as_ref()?.clone()))
    }

    fn build_journal_service(
        journal_sqlite_dao: Result<Arc<JournalSqliteDao>, String>,
    ) -> Result<JournalService<JournalSqliteDao>, String> {
        Ok(JournalService::new(journal_sqlite_dao.as_ref()?.clone()))
    }

    fn build_client_dao(
        database_manager: &Result<Arc<DatabaseManager>, String>,
    ) -> Result<ClientSqliteDao, String> {
        Ok(ClientSqliteDao::new(
            database_manager.as_ref()?.get_pool().clone(),
        ))
    }

    fn build_invoice_dao(
        database_manager: &Result<Arc<DatabaseManager>, String>,
    ) -> Result<InvoiceSqliteDao, String> {
        Ok(InvoiceSqliteDao::new(
            database_manager.as_ref()?.get_pool().clone(),
        ))
    }

    fn build_account_dao(
        database_manager: &Result<Arc<DatabaseManager>, String>,
    ) -> Result<AccountSqliteDao, String> {
        Ok(AccountSqliteDao::new(
            database_manager.as_ref()?.get_pool().clone(),
        ))
    }

    fn build_journal_dao(
        database_manager: &Result<Arc<DatabaseManager>, String>,
    ) -> Result<JournalSqliteDao, String> {
        Ok(JournalSqliteDao::new(
            database_manager.as_ref()?.get_pool().clone(),
        ))
    }

    async fn build_database_manager(
        configs: &Result<Arc<Configuration>, String>,
    ) -> Result<DatabaseManager, String> {
        match configs {
            Ok(config) => DatabaseManager::new(config.get_database_configuration()).await,
            Err(e) => Err(e.clone()),
        }
    }
}
