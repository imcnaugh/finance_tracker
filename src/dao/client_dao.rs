use crate::dao::error::DatabaseError;
use crate::model::NewClient;
use crate::model::client::Client;

pub trait ClientDao {
    fn create_client(new_client: NewClient) -> Result<Client, DatabaseError>;
    fn get_client_by_id(id: &str) -> Result<Option<Client>, DatabaseError>;
}
