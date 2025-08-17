use crate::dao::crud::Crud;
use crate::dao::line_item_dao::LineItemDao;
use crate::model::line_item::LineItem;
use sqlx::{Executor, Sqlite};
use std::error::Error;

pub struct LineItemSqliteDao;

const INSERT_SQL: &str = r#"
INSERT INTO line_item (
    id,
    description,
    quantity,
    unit_price_in_cents,
    invoice_id
) VALUES (?, ?, ?, ?, ?)
"#;

const SELECT_BY_ID_SQL: &str = r#"
SELECT
    id,
    description,
    quantity,
    unit_price_in_cents,
    invoice_id,
    created_timestamp
FROM line_item
WHERE id = ?
"#;

const UPDATE_SQL: &str = r#"
UPDATE line_item
SET
    description = ?,
    quantity = ?,
    unit_price_in_cents = ?,
    invoice_id = ?
WHERE id = ?
"#;

const DELETE_SQL: &str = r#"
DELETE FROM line_item
WHERE id = ?
"#;

const SELECT_LINE_ITEMS_FOR_INVOICE_SQL: &str = r#"
SELECT
    id,
    description,
    quantity,
    unit_price_in_cents,
    invoice_id,
    created_timestamp
FROM line_item
WHERE invoice_id = ?
"#;

impl LineItemSqliteDao {
    pub fn new() -> Self {
        Self {}
    }
}

impl Crud<LineItem> for LineItemSqliteDao {
    type DB = Sqlite;

    async fn create<'e, E>(&self, executor: E, item: &LineItem) -> Result<(), Box<dyn Error>>
    where
        E: Executor<'e, Database = Self::DB>,
    {
        let query = sqlx::query(INSERT_SQL)
            .bind(item.get_id())
            .bind(item.get_description())
            .bind(item.get_quantity() as f64)
            .bind(item.get_unit_price_in_cents() as i32)
            .bind(item.get_invoice_id());

        query.execute(executor).await?;
        Ok(())
    }

    async fn read<'e, E>(&self, executor: E, id: &str) -> Result<Option<LineItem>, Box<dyn Error>>
    where
        E: Executor<'e, Database = Self::DB>,
    {
        let item = sqlx::query_as::<_, LineItem>(SELECT_BY_ID_SQL)
            .bind(id)
            .fetch_optional(executor)
            .await?;

        Ok(item)
    }

    async fn update<'e, E>(
        &self,
        executor: E,
        id: &str,
        item: &LineItem,
    ) -> Result<(), Box<dyn Error>>
    where
        E: Executor<'e, Database = Self::DB>,
    {
        let query = sqlx::query(UPDATE_SQL)
            .bind(item.get_description())
            .bind(item.get_quantity())
            .bind(item.get_unit_price_in_cents())
            .bind(item.get_invoice_id())
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

impl LineItemDao for LineItemSqliteDao {
    async fn get_line_items_for_invoice<'e, E>(
        &self,
        executor: E,
        invoice_id: &str,
    ) -> Result<Vec<LineItem>, Box<dyn Error>>
    where
        E: Executor<'e, Database = Self::DB>,
    {
        let items = sqlx::query_as::<_, LineItem>(SELECT_LINE_ITEMS_FOR_INVOICE_SQL)
            .bind(invoice_id)
            .fetch_all(executor)
            .await?;

        Ok(items)
    }
}
