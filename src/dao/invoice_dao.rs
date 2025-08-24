use crate::model::invoice::Invoice;
use crate::model::{InvoiceSearch, NewInvoice};

pub trait InvoiceDao {
    async fn create_invoice(&self, new_invoice: &NewInvoice) -> Result<Invoice, sqlx::Error>;
    async fn get_invoice(&self, id: &str) -> Result<Option<Invoice>, sqlx::Error>;
    async fn search_invoices(
        &self,
        search_terms: InvoiceSearch,
    ) -> Result<Vec<Invoice>, sqlx::Error>;
}
