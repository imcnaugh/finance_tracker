use crate::dao::client_dao::ClientDao;
use crate::dao::crud::Crud;
use crate::model::client::Client;
use sqlx::{Executor, Sqlite};
use std::error::Error;

pub struct ClientSqliteDao;

const INSERT_SQL: &str = r#"
INSERT INTO client (
    id,
    name,
) VALUES (?, ?)
"#;

const SELECT_BY_ID_SQL: &str = r#"
SELECT
    id,
    name,
    created_timestamp
FROM client
WHERE id = ?
"#;

const UPDATE_SQL: &str = r#"
UPDATE client
SET
    name = ?
WHERE id = ?
"#;

const DELETE_SQL: &str = r#"
DELETE FROM client
WHERE id = ?
"#;

impl ClientSqliteDao {
    pub fn new() -> Self {
        Self
    }
}

impl Crud<Client, Sqlite> for ClientSqliteDao {
    async fn create<'e, E>(&self, executor: E, item: &Client) -> Result<(), Box<dyn Error>>
    where
        E: Executor<'e, Database = Sqlite>,
    {
        let query = sqlx::query(INSERT_SQL)
            .bind(item.get_id())
            .bind(item.get_name());

        query.execute(executor).await?;
        Ok(())
    }

    async fn read<'e, E>(&self, executor: E, id: &str) -> Result<Option<Client>, Box<dyn Error>>
    where
        E: Executor<'e, Database = Sqlite>,
    {
        let item = sqlx::query_as::<_, Client>(SELECT_BY_ID_SQL)
            .bind(id)
            .fetch_optional(executor)
            .await?;

        Ok(item)
    }

    async fn update<'e, E>(
        &self,
        executor: E,
        id: &str,
        item: &Client,
    ) -> Result<(), Box<dyn Error>>
    where
        E: Executor<'e, Database = Sqlite>,
    {
        let query = sqlx::query(UPDATE_SQL).bind(item.get_name()).bind(id);

        query.execute(executor).await?;
        Ok(())
    }

    async fn delete<'e, E>(&self, executor: E, id: &str) -> Result<(), Box<dyn Error>>
    where
        E: Executor<'e, Database = Sqlite>,
    {
        let query = sqlx::query(DELETE_SQL).bind(id);

        query.execute(executor).await?;
        Ok(())
    }
}

impl ClientDao<Sqlite> for ClientSqliteDao {}
