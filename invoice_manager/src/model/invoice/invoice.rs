use crate::model::line_item::LineItem;
use crate::model::{InvoiceStatus, NewInvoice};
use chrono::{DateTime, Utc};
use utilities::Error;
use utilities::utils::{generate_new_id, timestamp_to_date_time};

#[derive(Debug, sqlx::FromRow)]
pub struct Invoice {
    id: String,
    client_id: String,
    draft_date: i64,
    due_date: Option<i64>,
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
        due_date: Option<DateTime<Utc>>,
        sent_date: Option<DateTime<Utc>>,
        paid_date: Option<DateTime<Utc>>,
        cancelled_date: Option<DateTime<Utc>>,
        line_items: Vec<LineItem>,
    ) -> Self {
        Self {
            id,
            client_id,
            draft_date: draft_date.timestamp(),
            due_date: due_date.map(|d| d.timestamp()),
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

    pub fn get_draft_date(&self) -> Result<DateTime<Utc>, Error> {
        timestamp_to_date_time(self.draft_date)
    }

    pub fn get_sent_date(&self) -> Result<Option<DateTime<Utc>>, Error> {
        self.sent_date.map(timestamp_to_date_time).transpose()
    }

    pub fn get_due_date(&self) -> Result<Option<DateTime<Utc>>, Error> {
        self.due_date.map(timestamp_to_date_time).transpose()
    }

    pub fn get_paid_date(&self) -> Result<Option<DateTime<Utc>>, Error> {
        self.paid_date.map(timestamp_to_date_time).transpose()
    }

    pub fn get_cancelled_date(&self) -> Result<Option<DateTime<Utc>>, Error> {
        self.cancelled_date.map(timestamp_to_date_time).transpose()
    }

    pub fn get_status(&self) -> Result<InvoiceStatus, Error> {
        if self.cancelled_date.is_some() {
            Ok(InvoiceStatus::CANCELLED)
        } else if self.paid_date.is_some() {
            Ok(InvoiceStatus::PAID)
        } else if self
            .get_due_date()?
            .map(|d| d < Utc::now())
            .unwrap_or(false)
        {
            Ok(InvoiceStatus::OVERDUE)
        } else if self.sent_date.is_some() {
            Ok(InvoiceStatus::SENT)
        } else {
            Ok(InvoiceStatus::DRAFT)
        }
    }

    pub fn get_line_items(&self) -> &Vec<LineItem> {
        &self.line_items
    }

    pub fn set_line_items(&mut self, line_items: Vec<LineItem>) {
        self.line_items = line_items;
    }
}

impl From<&NewInvoice> for Invoice {
    fn from(value: &NewInvoice) -> Self {
        Self {
            id: generate_new_id(),
            client_id: value.get_client_id().into(),
            draft_date: Utc::now().timestamp(),
            due_date: None,
            sent_date: None,
            paid_date: None,
            cancelled_date: None,
            line_items: vec![],
        }
    }
}
