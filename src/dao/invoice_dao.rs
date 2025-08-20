use crate::model::NewInvoice;

pub trait InvoiceDao {
    async fn create_invoice(&self, new_invoice: NewInvoice);
    async fn get_invoice(&self, id: i32);
    async fn search_invoices(&self);
}
