use crate::dao::crud::Crud;
use crate::dao::sqlite::line_item_sqlite_dao::LineItemSqliteDao;
use crate::dao::sqlite::sqlite_connection;
use crate::model::NewLineItem;
use crate::model::line_item::LineItem;
use sqlx::Acquire;
use std::error::Error;

pub struct LineItemService {}

impl LineItemService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn log_new_line_item(new_line_item: NewLineItem) -> Result<(), Box<dyn Error>> {
        let dao = LineItemSqliteDao::new();
        let line_item = LineItem::from(new_line_item);

        let mut conn = sqlite_connection::get_pooled_connection().await;
        let mut tx = conn.begin().await?;

        dao.create(&mut *tx, &line_item)
            .await
            .expect("TODO: panic message");

        let read = dao.read(&mut *tx, line_item.get_id()).await.expect("");
        println!("{:?}", read);

        tx.commit().await?;

        Ok(())
    }
}
