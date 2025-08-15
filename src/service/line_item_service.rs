use crate::dao::crud::Crud;
use crate::dao::sqlite::sqlite_connection::get_pooled_connection;
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
        // let dao = LineItemSqliteDao::new();
        let line_item = LineItem::from(new_line_item);
        // dao.create(&line_item).expect("TODO: panic message");

        let conn = get_pooled_connection().await.acquire();

        Ok(())
    }
}
