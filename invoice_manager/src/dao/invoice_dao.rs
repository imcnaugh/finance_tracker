use crate::model::Invoice;
use crate::model::InvoiceStatus;
use crate::model::{InvoiceSearch, NewInvoice, NewLineItem};

pub trait InvoiceDao {
    fn create_invoice(
        &self,
        new_invoice: &NewInvoice,
    ) -> impl Future<Output = Result<Invoice, sqlx::Error>> + Send;
    fn get_invoice(
        &self,
        id: &str,
    ) -> impl Future<Output = Result<Option<Invoice>, sqlx::Error>> + Send;
    fn set_invoice_status_timestamp(
        &self,
        id: &str,
        sent_date: i64,
        status: InvoiceStatus,
    ) -> impl Future<Output = Result<Invoice, sqlx::Error>> + Send;
    fn search_invoices(
        &self,
        search_terms: &InvoiceSearch,
    ) -> impl Future<Output = Result<Vec<Invoice>, sqlx::Error>> + Send;

    fn create_line_item(
        &self,
        invoice_id: &str,
        new_line_item: &NewLineItem,
    ) -> impl Future<Output = Result<Invoice, sqlx::Error>> + Send;

    fn delete_line_item(
        &self,
        invoice_id: &str,
        line_item_id: &str,
    ) -> impl Future<Output = Result<Invoice, sqlx::Error>> + Send;
}
