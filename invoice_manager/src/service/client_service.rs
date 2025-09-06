use crate::dao::client_dao::ClientDao;
use crate::dao::sqlite::client_sqlite_dao::ClientSqliteDao;
use crate::model::Client;
use crate::model::NewClient;

pub struct ClientService {
    client_dao: ClientSqliteDao,
}

impl ClientService {
    pub fn new() -> Self {
        let client_dao = ClientSqliteDao::new();
        Self { client_dao }
    }

    pub async fn create_client(&self, new_client: NewClient) -> Result<Client, String> {
        let client = self
            .client_dao
            .create_client(new_client)
            .await
            .map_err(|e| e.to_string())?;

        Ok(client)
    }

    pub async fn get_client_by_id(&self, id: &str) -> Result<Client, String> {
        self.client_dao
            .get_client_by_id(id)
            .await
            .map_err(|e| e.to_string())
            .and_then(|opt| opt.ok_or_else(|| "client not found".to_string()))
    }

    pub async fn get_all_clients(&self) -> Result<Vec<Client>, String> {
        self.client_dao
            .get_all_clients()
            .await
            .map_err(|e| e.to_string())
    }
}

impl Default for ClientService {
    fn default() -> Self {
        Self::new()
    }
}
