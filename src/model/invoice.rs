use crate::model::invoice_status::InvoiceStatus;
use crate::model::line_item::LineItem;
use crate::model::new_invoice::NewInvoice;
use crate::utils::generate_new_id;
use chrono::{DateTime, TimeZone, Utc};

#[derive(Debug, sqlx::FromRow)]
pub struct Invoice {
    id: String,
    client_id: String,
    draft_date: i64,
    sent_date: Option<i64>,
    paid_date: Option<i64>,
    cancelled_date: Option<i64>,

    #[sqlx(skip)]
    line_items: Vec<LineItem>,
}

impl Invoice {
    pub fn new(
        id: String,
        client_id: String,
        draft_date: DateTime<Utc>,
        sent_date: Option<DateTime<Utc>>,
        paid_date: Option<DateTime<Utc>>,
        cancelled_date: Option<DateTime<Utc>>,
        line_items: Vec<LineItem>,
    ) -> Self {
        Self {
            id,
            client_id,
            draft_date: draft_date.timestamp(),
            sent_date: sent_date.map(|d| d.timestamp()),
            paid_date: paid_date.map(|d| d.timestamp()),
            cancelled_date: cancelled_date.map(|d| d.timestamp()),
            line_items,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_client_id(&self) -> &str {
        &self.client_id
    }

    pub fn get_draft_date(&self) -> DateTime<Utc> {
        // Safe conversion from epoch seconds; fall back to epoch start if invalid
        Utc.timestamp_opt(self.draft_date, 0)
            .single()
            .unwrap_or_else(|| Utc.timestamp(0, 0))
    }

    pub fn get_sent_date(&self) -> Option<DateTime<Utc>> {
        self.sent_date
            .and_then(|s| Utc.timestamp_opt(s, 0).single())
    }

    pub fn get_paid_date(&self) -> Option<DateTime<Utc>> {
        self.paid_date
            .and_then(|s| Utc.timestamp_opt(s, 0).single())
    }

    pub fn get_cancelled_date(&self) -> Option<DateTime<Utc>> {
        self.cancelled_date
            .and_then(|s| Utc.timestamp_opt(s, 0).single())
    }

    pub fn get_status(&self) -> InvoiceStatus {
        if self.cancelled_date.is_some() {
            InvoiceStatus::CANCELLED
        } else if self.paid_date.is_some() {
            InvoiceStatus::PAID
        } else if self.sent_date.is_some() {
            InvoiceStatus::SENT
        } else {
            InvoiceStatus::DRAFT
        }
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
            draft_date: Utc::now().timestamp(),
            sent_date: None,
            paid_date: None,
            cancelled_date: None,
            line_items: vec![],
        }
    }
}
