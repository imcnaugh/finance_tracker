use crate::model::NewLineItem;
use crate::utils::generate_new_id;
use chrono::LocalResult::{Ambiguous, Single};
use chrono::{DateTime, LocalResult, TimeZone, Utc};

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
    pub(crate) fn new(
        id: String,
        description: String,
        unit_price_in_cents: i32,
        quantity: f64,
        invoice_id: String,
        created_timestamp: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            description,
            unit_price_in_cents,
            quantity,
            invoice_id,
            created_timestamp: created_timestamp.timestamp(),
        }
    }

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
        let unit_price_in_cents = (value.0.get_unit_price() * 100.0) as i32;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::NewLineItem;

    #[test]
    fn test_from_new_line_item() {
        let new_line_item = NewLineItem::new("Test Item".into(), 10.0, 100.0);

        let line_item = LineItem::from((&new_line_item, "1234567890"));

        assert_eq!(line_item.description, "Test Item");
        assert_eq!(line_item.unit_price_in_cents, 10000);
        assert_eq!(line_item.quantity, 10.0);
        assert_eq!(line_item.id.len(), 10);
        assert_eq!(line_item.invoice_id, "1234567890");
    }
}
