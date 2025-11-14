use invoice_manager::model::Invoice;
use utilities::observer::event::Event;

pub struct InvoiceSentEvent {
    invoice: Invoice,
}

impl InvoiceSentEvent {
    fn new(invoice: Invoice) -> Self {
        Self { invoice }
    }

    pub fn get_invoice(&self) -> &Invoice {
        &self.invoice
    }
}

impl Event for InvoiceSentEvent {}
