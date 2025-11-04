use crate::config_service::get_config;
use crate::database::DatabaseManager;
use crate::sqlite_dao::client_sqlite_dao::ClientSqliteDao;
use invoice_manager::service::ClientService;
use std::sync::Arc;

struct Context {
    client_service: Option<Arc<ClientService<ClientSqliteDao>>>,
}

impl Context {
    pub async fn new() -> Self {
        let db_manager = Self::build_database_manager().await;
        let client_dao = Self::build_client_dao(db_manager).await;
        let client_service = Self::build_client_service(client_dao).await;

        Self { client_service }
    }

    pub fn get_client_service_ref(&self) -> Option<&ClientService<ClientSqliteDao>> {
        match &self.client_service {
            None => None,
            Some(service) => Some(service.as_ref()),
        }
    }

    pub fn get_client_service(&self) -> Result<Arc<ClientService<ClientSqliteDao>>, String> {
        match &self.client_service {
            None => Err("Client service is not set".to_string()),
            Some(service) => Ok(service.clone()),
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

    async fn build_client_dao(
        database_manager: Option<Arc<DatabaseManager>>,
    ) -> Option<Arc<ClientSqliteDao>> {
        match database_manager {
            None => None,
            Some(db_manager) => Some(Arc::new(ClientSqliteDao::new(
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
