use crate::model::invoice::Invoice;
use crate::utils::generate_new_id;

pub struct InvoiceService {
    // invoice_dao: InvoiceSqlDao,
}

impl InvoiceService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_new_invoice(client_id: String) {
        let new_invoice = Invoice::new(generate_new_id(), client_id, Vec::new());
    }
}
