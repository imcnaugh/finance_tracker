use crate::dao::crud::Crud;
use crate::model::line_item::LineItem;
use sqlx::{Database, Executor};
use std::error::Error;

pub trait LineItemDao<DB: Database>: Crud<LineItem, DB> {
    async fn get_line_items_for_invoice<'e, E>(
        &self,
        executor: E,
        invoice_id: &str,
    ) -> Result<Vec<LineItem>, Box<dyn Error>>
    where
        E: Executor<'e>;
}
