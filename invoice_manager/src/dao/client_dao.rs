use crate::model::Client;
use crate::model::NewClient;

pub trait ClientDao {
    async fn create_client(&self, new_client: NewClient) -> Result<Client, sqlx::Error>;
    async fn get_client_by_id(&self, id: &str) -> Result<Option<Client>, sqlx::Error>;
    async fn get_all_clients(&self) -> Result<Vec<Client>, sqlx::Error>;
}
