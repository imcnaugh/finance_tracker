use crate::dao::client_dao::ClientDao;
use crate::dao::sqlite::client_sqlite_dao::ClientSqliteDao;
use crate::model::NewClient;
use crate::model::client::Client;
use crate::model::error::Error;

pub struct ClientService<'d> {
    client_dao: &'d ClientSqliteDao,
}

impl<'d> ClientService<'d> {
    pub fn new(client_dao: &'d ClientSqliteDao) -> Self {
        Self { client_dao }
    }

    pub async fn create_client(&self, new_client: NewClient) -> Result<Client, Error> {
        match self.client_dao.create_client(new_client).await {
            Ok(client) => Ok(client),
            Err(_) => Err(Error::new(
                "There was an unexpected error while creating the client".to_string(),
            )),
        }
    }

    pub async fn get_all_clients(&self) -> Result<Vec<Client>, Error> {
        match self.client_dao.get_all_clients().await {
            Ok(clients) => Ok(clients),
            Err(_) => Err(Error::new(
                "There was an unexpected error while getting all clients".to_string(),
            )),
        }
    }
}
