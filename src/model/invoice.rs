use crate::model::invoice_status::InvoiceStatus;
use crate::model::invoice_status::InvoiceStatus::DRAFT;
use crate::model::line_item::LineItem;
use crate::model::new_invoice::NewInvoice;
use crate::utils::generate_new_id;
use chrono::{DateTime, Local};

#[derive(Debug, sqlx::FromRow)]
pub struct Invoice {
    id: String,
    client_id: String,
    status: InvoiceStatus,

    #[sqlx(skip)]
    line_items: Vec<LineItem>,
}

impl Invoice {
    pub fn new(
        id: String,
        client_id: String,
        status: InvoiceStatus,
        line_items: Vec<LineItem>,
    ) -> Self {
        Self {
            id,
            client_id,
            status,
            line_items,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_client_id(&self) -> &str {
        &self.client_id
    }

    pub fn get_status(&self) -> &InvoiceStatus {
        &self.status
    }

    pub fn get_line_items(&self) -> &Vec<LineItem> {
        &self.line_items
    }
}

impl From<&NewInvoice> for Invoice {
    fn from(value: &NewInvoice) -> Self {
        Self {
            id: generate_new_id(),
            client_id: value.get_client_id().into(),
            status: DRAFT,
            line_items: Vec::new(),
        }
    }
}
