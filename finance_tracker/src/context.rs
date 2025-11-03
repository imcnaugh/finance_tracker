use std::sync::Arc;
use invoice_manager::service::ClientService;
use crate::config_service::get_config;
use crate::database::DatabaseManager;
use crate::sqlite_dao::client_sqlite_dao::ClientSqliteDao;

struct Context {
    database_manager: Option<DatabaseManager>,
    client_dao: Option<ClientSqliteDao>,
    client_service: Option<Arc<ClientService<ClientSqliteDao>>>,
}

impl Context {
    pub async fn new() -> Self {
        let db_manager = Self::build_database_manager().await;
        let client_dao = Self::build_client_dao(db_manager);



        Self {
            database_manager: None,
            client_dao: None,
            client_service: None,
        }
    }

    pub fn get_client_service(&self) -> Result<Arc<ClientService<ClientSqliteDao>>, String> {
        match &self.client_service {
            None => Err("Client service is not set".to_string()),
            Some(service) => Ok(service.clone())
        }
    }

    async fn build_client_service(client_sqlite_dao: ClientSqliteDao) -> Option<Arc<ClientService<ClientSqliteDao>>> {
        todo!()
    }

    async fn build_client_dao(database_manager: Option<Arc<DatabaseManager>>) -> Option<ClientSqliteDao>{
        match database_manager{
            None => None,
            Some(db_manager) => Some(ClientSqliteDao::new(db_manager.get_pool().clone()))
        }
    }

    async fn build_database_manager() -> Option<DatabaseManager> {
        let configuration = get_config().ok()?;
        let db_configs = configuration.get_database_configuration();
        DatabaseManager::new(db_configs).await.ok()
    }
}