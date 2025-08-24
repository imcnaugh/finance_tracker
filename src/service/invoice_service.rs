use crate::dao::invoice_dao::InvoiceDao;
use crate::dao::sqlite::invoice_sqlite_dao::InvoiceSqliteDao;
use crate::model::{InvoiceSearch, NewInvoice};
use crate::model::invoice::Invoice;

pub struct InvoiceService<ID>
where
    ID: InvoiceDao,
{
    invoice_dao: ID,
}

impl InvoiceService<InvoiceSqliteDao> {
    pub fn new() -> InvoiceService<InvoiceSqliteDao>
    {
        let invoice_dao = InvoiceSqliteDao::new();
        Self { invoice_dao }
    }
}

impl<ID: InvoiceDao> InvoiceService<ID> {

    pub(crate) fn new_with_dao(invoice_dao: ID) -> InvoiceService<ID>
    {
        Self { invoice_dao }
    }

    pub async fn create_new_invoice(&self, client_id: String) -> Result<Invoice, String> {
        let new_invoice = NewInvoice::new(client_id);

        let invoice = self.invoice_dao
            .create_invoice(&new_invoice)
            .await
            .map_err(|e| e.to_string())?;

        Ok(invoice)
    }

    pub async fn list_invoices(&self) -> Result<Vec<Invoice>, String> {
        let invoice_search = InvoiceSearch::new(
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );
        self.invoice_dao.search_invoices(invoice_search).await.map_err(|e| e.to_string())
    }
}
