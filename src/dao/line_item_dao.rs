use crate::dao::crud::Crud;
use crate::model::line_item::LineItem;
use std::error::Error;

pub trait LineItemDao: Crud<LineItem> {
    async fn get_line_items_for_invoice(
        &self,
        invoice_id: &str,
    ) -> Result<Vec<LineItem>, Box<dyn Error>>;
}
