use crate::model::Client;
use crate::model::NewClient;

pub trait ClientDao {
    fn create_client(
        &self,
        new_client: NewClient,
    ) -> impl Future<Output = Result<Client, sqlx::Error>>;
    fn get_client_by_id(
        &self,
        id: &str,
    ) -> impl Future<Output = Result<Option<Client>, sqlx::Error>>;
    fn get_all_clients(&self) -> impl Future<Output = Result<Vec<Client>, sqlx::Error>>;
}
