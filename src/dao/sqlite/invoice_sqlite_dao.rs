use crate::dao::crud::Crud;
use crate::dao::invoice_dao::InvoiceDao;
use crate::model::invoice::Invoice;
use sqlx::{Executor, Sqlite};
use std::error::Error;

pub struct InvoiceSqliteDao;

const INSERT_SQL: &str = r#"
INSERT INTO invoice (
    id,
    client_id,
    status
) VALUES (?, ?, ?)
"#;

const SELECT_BY_ID_SQL: &str = r#"
SELECT
    id,
    client_id,
    status,
    created_timestamp
FROM invoice
WHERE id = ?
"#;

const UPDATE_SQL: &str = r#"
UPDATE invoice
SET
    client_id = ?,
    status = ?,
WHERE id = ?
"#;

const DELETE_SQL: &str = r#"
DELETE FROM invoice
WHERE id = ?
"#;

impl InvoiceSqliteDao {
    pub fn new() -> Self {
        Self {}
    }
}

impl Crud<Invoice> for InvoiceSqliteDao {
    type DB = Sqlite;

    async fn create<'e, E>(&self, executor: E, item: &Invoice) -> Result<(), Box<dyn Error>>
    where
        E: Executor<'e, Database = Self::DB>,
    {
        let query = sqlx::query(INSERT_SQL)
            .bind(item.get_id())
            .bind(item.get_client_id())
            .bind(item.get_status());

        query.execute(executor).await?;
        Ok(())
    }

    async fn read<'e, E>(&self, executor: E, id: &str) -> Result<Option<Invoice>, Box<dyn Error>>
    where
        E: Executor<'e, Database = Self::DB>,
    {
        let item = sqlx::query_as::<_, Invoice>(SELECT_BY_ID_SQL)
            .bind(id)
            .fetch_optional(executor)
            .await?;

        Ok(item)
    }

    async fn update<'e, E>(
        &self,
        executor: E,
        id: &str,
        item: &Invoice,
    ) -> Result<(), Box<dyn Error>>
    where
        E: Executor<'e, Database = Self::DB>,
    {
        let query = sqlx::query(UPDATE_SQL)
            .bind(item.get_client_id())
            .bind(item.get_status())
            .bind(id);

        query.execute(executor).await?;
        Ok(())
    }

    async fn delete<'e, E>(&self, executor: E, id: &str) -> Result<(), Box<dyn Error>>
    where
        E: Executor<'e, Database = Self::DB>,
    {
        let query = sqlx::query(DELETE_SQL).bind(id);

        query.execute(executor).await?;
        Ok(())
    }
}

impl InvoiceDao for InvoiceSqliteDao {}
