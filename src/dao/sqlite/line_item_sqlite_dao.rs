use crate::dao::crud::Crud;
use crate::dao::line_item_dao::LineItemDao;
use crate::dao::sqlite::sqlite_connection::get_pooled_connection;
use crate::model::line_item::LineItem;
use std::error::Error;

pub struct LineItemSqliteDao;

impl LineItemSqliteDao {
    pub fn new() -> Self {
        Self {}
    }
}

impl Crud<LineItem> for LineItemSqliteDao {
    async fn create(&self, item: &LineItem) -> Result<(), Box<dyn Error>> {
        let conn = get_pooled_connection().await;
        let mut conn = conn.acquire().await?;
        let query = sqlx::query("INSERT INTO line_item (id, description,  quantity, unit_price_in_cents, invoice_id) VALUES (?, ?, ?, ?, ?)")
            .bind(item.get_id())
            .bind(item.get_description())
            .bind(item.get_quantity() as f64)
            .bind(item.get_unit_price_in_cents() as i32)
            .bind(item.get_invoice_id());

        query.execute(&mut *conn).await?;
        Ok(())
    }

    async fn read(&self, id: &str) -> Result<LineItem, Box<dyn Error>> {
        todo!()
    }

    async fn update(&self, id: &str, item: &LineItem) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    async fn delete(&self, id: &str) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

impl LineItemDao for LineItemSqliteDao {
    async fn get_line_items_for_invoice(
        &self,
        invoice_id: &str,
    ) -> Result<Vec<LineItem>, Box<dyn Error>> {
        todo!()
    }
}
