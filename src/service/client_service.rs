use sqlx::{Connection, Executor};
use crate::dao::crud::Crud;
use crate::dao::sqlite::client_sqlite_dao::ClientSqliteDao;
use crate::dao::sqlite::sqlite_connection;
use crate::model::client::Client;
use crate::utils::generate_new_id;

pub struct ClientService<'d> {
    client_dao: &'d ClientSqliteDao,
}

impl<'d> ClientService<'d> {
    pub fn new(client_dao: &'d ClientSqliteDao) -> Self {
        Self { client_dao }
    }

    pub async fn create_new_client(&self, new_client_name: &str) {
        let new_client = Client::new(
            generate_new_id(),
            new_client_name.into()
        );

        let mut exe = sqlite_connection::get_pooled_connection().await;
        let mut conn = exe.begin().await.expect("");
        self.client_dao.create(&mut *conn, &new_client);
        &conn.commit().await.expect("");
    }
}
