use crate::model::invoice_status::InvoiceStatus;
use crate::model::line_item::LineItem;

#[derive(Debug, sqlx::FromRow)]
pub(crate) struct Invoice {
    id: String,
    client_id: String,
    status: InvoiceStatus,
    line_items: Vec<LineItem>,
}

impl Invoice {
    pub fn new(id: String, client_id: String, line_items: Vec<LineItem>) -> Self {
        Self {
            id,
            client_id,
            status: InvoiceStatus::DRAFT,
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
