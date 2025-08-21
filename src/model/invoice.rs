use chrono::{DateTime, Utc};
use crate::model::invoice_status::InvoiceStatus;
use crate::model::invoice_status::InvoiceStatus::DRAFT;
use crate::model::line_item::LineItem;
use crate::model::new_invoice::NewInvoice;
use crate::utils::generate_new_id;

#[derive(Debug, sqlx::FromRow)]
pub struct Invoice {
    id: String,
    client_id: String,
    current_status: InvoiceStatus,

    #[sqlx(skip)]
    line_items: Vec<LineItem>,
    history: Vec<InvoiceHistory>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct InvoiceHistory {
    status: InvoiceStatus,
    timestamp: DateTime<Utc>,
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
            current_status: status,
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
        &self.current_status
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
            current_status: DRAFT,
            line_items: Vec::new(),
        }
    }
}
