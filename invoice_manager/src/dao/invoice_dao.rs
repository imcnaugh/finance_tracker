use crate::model::invoice::Invoice;
use crate::model::line_item::LineItem;
use crate::model::{InvoiceSearch, NewInvoice, NewLineItem};

pub trait InvoiceDao {
    async fn create_invoice(&self, new_invoice: &NewInvoice) -> Result<Invoice, sqlx::Error>;
    async fn get_invoice(&self, id: &str) -> Result<Option<Invoice>, sqlx::Error>;
    async fn set_invoice_sent_timestamp(&self, id: &str, sent_date: i64)
    -> Result<(), sqlx::Error>;
    async fn search_invoices(
        &self,
        search_terms: &InvoiceSearch,
    ) -> Result<Vec<Invoice>, sqlx::Error>;

    async fn create_line_item(
        &self,
        invoice_id: &str,
        new_line_item: &NewLineItem,
    ) -> Result<LineItem, sqlx::Error>;
}
