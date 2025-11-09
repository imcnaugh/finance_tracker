use crate::command_handler::account::AccountCommandHandler;
use crate::command_handler::client::ClientCommandHandler;
use crate::command_handler::invoice::InvoiceCommandHandler;
use crate::command_handler::journal::JournalCommandHandler;
use crate::config_service::get_config;
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
    client_service: Option<Arc<ClientService<ClientSqliteDao>>>,
    invoice_service: Option<Arc<InvoiceService<InvoiceSqliteDao>>>,
    account_service: Option<Arc<AccountService<AccountSqliteDao>>>,
    journal_service: Option<Arc<JournalService<JournalSqliteDao>>>,
}

impl Context {
    pub async fn new() -> Self {
        let db_manager = Self::build_database_manager().await;
        let client_dao = Self::build_client_dao(&db_manager).await;
        let client_service = Self::build_client_service(client_dao).await;

        let invoice_dao = Self::build_invoice_dao(&db_manager).await;
        let invoice_service = Self::build_invoice_service(invoice_dao, Some(prompt_confirm)).await;

        let account_dao = Self::build_account_dao(&db_manager).await;
        let account_service = Self::build_account_service(account_dao).await;

        let journal_dao = Self::build_journal_dao(&db_manager).await;
        let journal_service = Self::build_journal_service(journal_dao).await;

        Self {
            client_service,
            invoice_service,
            account_service,
            journal_service,
        }
    }

    pub fn get_client_command_handler(&self) -> Result<ClientCommandHandler, String> {
        match &self.client_service {
            None => Err("Client service is not set".to_string()),
            Some(service) => Ok(ClientCommandHandler::new(service.clone())),
        }
    }

    pub fn get_invoice_command_handler(&self) -> Result<InvoiceCommandHandler, String> {
        match (&self.invoice_service, &self.client_service) {
            (Some(invoice_service), Some(client_service)) => Ok(InvoiceCommandHandler::new(
                client_service.clone(),
                invoice_service.clone(),
                get_config()?,
            )),
            _ => Err("Invoice service is not set".to_string()),
        }
    }

    pub fn get_account_command_handler(&self) -> Result<AccountCommandHandler, String> {
        match &self.account_service {
            None => Err("Account service is not set".to_string()),
            Some(service) => Ok(AccountCommandHandler::new(service.clone())),
        }
    }

    pub fn get_journal_command_handler(&self) -> Result<JournalCommandHandler, String> {
        match &self.journal_service {
            None => Err("Journal service is not set".to_string()),
            Some(service) => Ok(JournalCommandHandler::new(service.clone())),
        }
    }

    async fn build_client_service(
        client_sqlite_dao: Option<Arc<ClientSqliteDao>>,
    ) -> Option<Arc<ClientService<ClientSqliteDao>>> {
        match client_sqlite_dao {
            None => None,
            Some(client_sqlite_dao) => {
                Some(Arc::new(ClientService::new(client_sqlite_dao.clone())))
            }
        }
    }

    async fn build_invoice_service(
        invoice_sqlite_dao: Option<Arc<InvoiceSqliteDao>>,
        confirm_fn: Option<fn(&str) -> bool>,
    ) -> Option<Arc<InvoiceService<InvoiceSqliteDao>>> {
        match invoice_sqlite_dao {
            None => None,
            Some(invoice_sqlite_dao) => Some(Arc::new(InvoiceService::new(
                confirm_fn,
                invoice_sqlite_dao.clone(),
            ))),
        }
    }

    async fn build_account_service(
        account_sqlite_dao: Option<Arc<AccountSqliteDao>>,
    ) -> Option<Arc<AccountService<AccountSqliteDao>>> {
        match account_sqlite_dao {
            None => None,
            Some(account_sqlite_dao) => {
                Some(Arc::new(AccountService::new(account_sqlite_dao.clone())))
            }
        }
    }

    async fn build_journal_service(
        journal_sqlite_dao: Option<Arc<JournalSqliteDao>>,
    ) -> Option<Arc<JournalService<JournalSqliteDao>>> {
        match journal_sqlite_dao {
            None => None,
            Some(journal_sqlite_dao) => {
                Some(Arc::new(JournalService::new(journal_sqlite_dao.clone())))
            }
        }
    }

    async fn build_client_dao(
        database_manager: &Option<Arc<DatabaseManager>>,
    ) -> Option<Arc<ClientSqliteDao>> {
        match database_manager {
            None => None,
            Some(db_manager) => Some(Arc::new(ClientSqliteDao::new(
                db_manager.get_pool().clone(),
            ))),
        }
    }

    async fn build_invoice_dao(
        database_manager: &Option<Arc<DatabaseManager>>,
    ) -> Option<Arc<InvoiceSqliteDao>> {
        match database_manager {
            None => None,
            Some(db_manager) => Some(Arc::new(InvoiceSqliteDao::new(
                db_manager.get_pool().clone(),
            ))),
        }
    }

    async fn build_account_dao(
        database_manager: &Option<Arc<DatabaseManager>>,
    ) -> Option<Arc<AccountSqliteDao>> {
        match database_manager {
            None => None,
            Some(db_manager) => Some(Arc::new(AccountSqliteDao::new(
                db_manager.get_pool().clone(),
            ))),
        }
    }

    async fn build_journal_dao(
        database_manager: &Option<Arc<DatabaseManager>>,
    ) -> Option<Arc<JournalSqliteDao>> {
        match database_manager {
            None => None,
            Some(db_manager) => Some(Arc::new(JournalSqliteDao::new(
                db_manager.get_pool().clone(),
            ))),
        }
    }

    async fn build_database_manager() -> Option<Arc<DatabaseManager>> {
        let configuration = get_config().ok()?;
        let db_configs = configuration.get_database_configuration();
        let db_manager = DatabaseManager::new(db_configs).await.ok()?;
        Some(Arc::new(db_manager))
    }
}
