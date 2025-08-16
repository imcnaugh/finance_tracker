use crate::dao::crud::Crud;
use crate::dao::line_item_dao::LineItemDao;
use crate::model::line_item::LineItem;
use sqlx::{Executor, Sqlite};
use std::error::Error;

pub struct LineItemSqliteDao;

const INSERT_QUERY: &str = r#"
INSERT INTO line_item (
    id,
    description,
    quantity,
    unit_price_in_cents,
    invoice_id
) VALUES (?, ?, ?, ?, ?)
"#;

const SELECT_BY_ID_QUERY: &str = r#"
SELECT
    id,
    description,
    quantity,
    unit_price_in_cents,
    invoice_id,
    created_date
FROM line_item
WHERE id = ?
"#;

impl LineItemSqliteDao {
    pub fn new() -> Self {
        Self {}
    }
}

impl Crud<LineItem, Sqlite> for LineItemSqliteDao {
    async fn create<'e, E>(&self, executor: E, item: &LineItem) -> Result<(), Box<dyn Error>>
    where
        E: Executor<'e, Database = Sqlite>,
    {
        let query = sqlx::query(INSERT_QUERY)
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
        E: Executor<'e, Database = Sqlite>,
    {
        let item = sqlx::query_as::<_, LineItem>(SELECT_BY_ID_QUERY)
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
        E: Executor<'e, Database = Sqlite>,
    {
        todo!()
    }

    async fn delete<'e, E>(&self, executor: E, id: &str) -> Result<(), Box<dyn Error>>
    where
        E: Executor<'e, Database = Sqlite>,
    {
        todo!()
    }
}

impl LineItemDao<Sqlite> for LineItemSqliteDao {
    async fn get_line_items_for_invoice<'e, E>(
        &self,
        executor: E,
        invoice_id: &str,
    ) -> Result<Vec<LineItem>, Box<dyn Error>>
    where
        E: Executor<'e>,
    {
        todo!()
    }
}
