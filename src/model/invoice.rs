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

    #[sqlx(skip)]
    history: Vec<InvoiceHistory>,
}

#[derive(Debug, sqlx::FromRow, Copy, Clone)]
pub struct InvoiceHistory {
    status: InvoiceStatus,
    timestamp: DateTime<Utc>,
}

impl Invoice {
    pub fn new(
        id: String,
        client_id: String,
        line_items: Vec<LineItem>,
        history: Vec<InvoiceHistory>,
    ) -> Self {
        let mut history = history.clone();
        history.sort_by(|a, b|b.timestamp.cmp(&a.timestamp));
        let current_status = match history.clone().pop() {
            Some(value) => value.status,
            None => DRAFT,
        };

        Self {
            id,
            client_id,
            current_status,
            line_items,
            history: history.clone(),
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
            history: vec![InvoiceHistory{
                status: DRAFT,
                timestamp: Utc::now(),
            }],
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_invoice() {
        let new_invoice = Invoice::new(
            generate_new_id(),
            "1234567890".into(),
            vec![],
            vec![],
        );

        assert_eq!(new_invoice.current_status, DRAFT);
    }

    #[test]
    fn test_new_invoice_1_history_item() {
        let new_invoice = Invoice::new(
            generate_new_id(),
            "1234567890".into(),
            vec![],
            vec![InvoiceHistory{
                status: InvoiceStatus::OVERDUE,
                timestamp: Utc::now(),
            }],
        );

        assert_eq!(new_invoice.current_status, InvoiceStatus::OVERDUE);
    }

    #[test]
    fn test_new_invoice_2_history_item() {
        let new_invoice = Invoice::new(
            generate_new_id(),
            "1234567890".into(),
            vec![],
            vec![InvoiceHistory{
                status: InvoiceStatus::OVERDUE,
                timestamp: Utc::now(),
            }, InvoiceHistory{
                status: InvoiceStatus::PAID,
                timestamp: Utc::now() - chrono::Duration::days(1),
            }],
        );

        assert_eq!(new_invoice.current_status, InvoiceStatus::PAID);
    }
}