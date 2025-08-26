use crate::dao::invoice_dao::InvoiceDao;
use crate::dao::sqlite::invoice_sqlite_dao::InvoiceSqliteDao;
use crate::model::invoice::Invoice;
use crate::model::line_item::LineItem;
use crate::model::{InvoiceSearch, NewInvoice, NewLineItem};

pub struct InvoiceService<ID>
where
    ID: InvoiceDao,
{
    invoice_dao: ID,
}

impl InvoiceService<InvoiceSqliteDao> {
    pub fn new() -> InvoiceService<InvoiceSqliteDao> {
        let invoice_dao = InvoiceSqliteDao::new();
        Self { invoice_dao }
    }
}

impl<ID: InvoiceDao> InvoiceService<ID> {
    pub(crate) fn new_with_dao(invoice_dao: ID) -> InvoiceService<ID> {
        Self { invoice_dao }
    }

    pub async fn create_new_invoice(&self, client_id: String) -> Result<Invoice, String> {
        let new_invoice = NewInvoice::new(client_id);

        let invoice = self
            .invoice_dao
            .create_invoice(&new_invoice)
            .await
            .map_err(|e| e.to_string())?;

        Ok(invoice)
    }

    pub async fn search_invoices(
        &self,
        search_terms: Option<InvoiceSearch>,
    ) -> Result<Vec<Invoice>, String> {
        let search_terms = search_terms.unwrap_or_default();
        self.invoice_dao
            .search_invoices(&search_terms)
            .await
            .map_err(|e| e.to_string())
    }

    pub async fn add_line_item_to_invoice(
        &self,
        invoice_id: &str,
        new_line_item: &NewLineItem,
    ) -> Result<LineItem, String> {
        self.invoice_dao
            .create_line_item(invoice_id, new_line_item)
            .await
            .map_err(|e| e.to_string())
    }
}
