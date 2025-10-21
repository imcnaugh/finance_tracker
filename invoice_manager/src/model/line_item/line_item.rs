use crate::model::NewLineItem;
use chrono::LocalResult::{Ambiguous, Single};
use chrono::{DateTime, LocalResult, TimeZone, Utc};
use utilities::utils::generate_new_id;

#[derive(Debug, sqlx::FromRow)]
pub struct LineItem {
    id: String,
    description: String,
    unit_price_in_cents: i32,
    quantity: f64,
    invoice_id: String,
    created_timestamp: i64,
}

impl LineItem {
    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_unit_price_in_cents(&self) -> i32 {
        self.unit_price_in_cents
    }

    pub fn get_quantity(&self) -> f64 {
        self.quantity
    }

    pub fn get_total_in_cents(&self) -> i32 {
        self.unit_price_in_cents * self.quantity as i32
    }

    pub fn get_invoice_id(&self) -> &str {
        &self.invoice_id
    }

    pub fn get_created_timestamp(&self) -> DateTime<Utc> {
        match Utc.timestamp_opt(self.created_timestamp, 0) {
            Single(d) => d,
            Ambiguous(e, _) => e,
            LocalResult::None => Utc::now(),
        }
    }
}

impl From<(&NewLineItem, &str)> for LineItem {
    fn from(value: (&NewLineItem, &str)) -> Self {
        let unit_price_in_cents = (value.0.get_unit_price() * 100.0).round() as i32;

        Self {
            id: generate_new_id(),
            description: value.0.get_description().into(),
            unit_price_in_cents,
            quantity: value.0.get_quantity(),
            invoice_id: value.1.into(),
            created_timestamp: Utc::now().timestamp(),
        }
    }
}
