use invoice_manager::dao::client_dao::ClientDao;
use invoice_manager::model::Client;
use invoice_manager::model::NewClient;
use sqlx::{Executor, Pool, Sqlite};

pub struct ClientSqliteDao {
    pool: Pool<Sqlite>,
}

const INSERT_SQL: &str = r#"
INSERT INTO client (
    id,
    name,
    address,
    phone,
    invoice_email
) VALUES (?, ?, ?, ?, ?)
"#;

const SELECT_BY_ID_SQL: &str = r#"
SELECT
    id,
    name,
    address,
    phone,
    invoice_email,
    created_timestamp
FROM client
WHERE id = ?
"#;

const SELECT_ALL_SQL: &str = r#"
SELECT
    id,
    name,
    address,
    phone,
    invoice_email,
    created_timestamp
FROM client
"#;

impl ClientSqliteDao {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    async fn create<'e, E>(&self, executor: E, item: &Client) -> Result<(), sqlx::Error>
    where
        E: Executor<'e, Database = Sqlite>,
    {
        let query = sqlx::query(INSERT_SQL)
            .bind(item.get_id())
            .bind(item.get_name())
            .bind(item.get_address())
            .bind(item.get_phone())
            .bind(item.get_invoice_email());

        query.execute(executor).await?;
        Ok(())
    }

    async fn read<'e, E>(&self, executor: E, id: &str) -> Result<Option<Client>, sqlx::Error>
    where
        E: Executor<'e, Database = Sqlite>,
    {
        let item = sqlx::query_as::<_, Client>(SELECT_BY_ID_SQL)
            .bind(id)
            .fetch_optional(executor)
            .await?;

        Ok(item)
    }

    async fn read_all<'e, E>(&self, executor: E) -> Result<Vec<Client>, sqlx::Error>
    where
        E: Executor<'e, Database = Sqlite>,
    {
        let query = sqlx::query_as::<_, Client>(SELECT_ALL_SQL);
        query.fetch_all(executor).await
    }
}

impl ClientDao for ClientSqliteDao {
    async fn create_client(&self, new_client: NewClient) -> Result<Client, sqlx::Error> {
        let mut conn = self.pool.acquire().await?;

        let client = Client::from(new_client);

        self.create(&mut *conn, &client).await?;
        Ok(client)
    }

    async fn get_client_by_id(&self, id: &str) -> Result<Option<Client>, sqlx::Error> {
        let mut conn = self.pool.acquire().await?;
        self.read(&mut *conn, id).await
    }

    async fn get_all_clients(&self) -> Result<Vec<Client>, sqlx::Error> {
        let mut conn = self.pool.acquire().await?;
        self.read_all(&mut *conn).await
    }
}
