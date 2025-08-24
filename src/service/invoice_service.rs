use crate::dao::invoice_dao::InvoiceDao;
use crate::dao::sqlite::invoice_sqlite_dao::InvoiceSqliteDao;
use crate::model::invoice::Invoice;
use crate::model::invoice_status::InvoiceStatus;
use crate::model::invoice_status::InvoiceStatus::DRAFT;
use crate::model::{InvoiceSearch, NewInvoice};
use chrono::{Duration, Utc};

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

    pub async fn list_invoices(&self) -> Result<Vec<Invoice>, String> {
        let mut invoice_search = InvoiceSearch::new();
        invoice_search.set_client_id("9e7ec31e1f");
        invoice_search.set_status(&DRAFT);
        // invoice_search.set_sent_date_range(Utc::now() - Duration::days(1), Utc::now());
        invoice_search.set_draft_date_range(Utc::now() - Duration::days(1), Utc::now());
        self.invoice_dao
            .search_invoices(invoice_search)
            .await
            .map_err(|e| e.to_string())
    }
}
