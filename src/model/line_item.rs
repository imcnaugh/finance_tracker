use crate::model::NewLineItem;
use crate::utils::generate_new_id;

#[derive(Debug, sqlx::FromRow)]
pub(crate) struct LineItem {
    id: String,
    description: String,
    unit_price_in_cents: i32,
    quantity: f64,
    invoice_id: String,
}

impl LineItem {
    pub(crate) fn new(
        id: String,
        description: String,
        unit_price_in_cents: i32,
        quantity: f64,
        invoice_id: String,
    ) -> Self {
        Self {
            id,
            description,
            unit_price_in_cents,
            quantity,
            invoice_id,
        }
    }

    pub(crate) fn get_id(&self) -> &str {
        &self.id
    }

    pub(crate) fn get_description(&self) -> &str {
        &self.description
    }

    pub(crate) fn get_unit_price_in_cents(&self) -> i32 {
        self.unit_price_in_cents
    }

    pub(crate) fn get_quantity(&self) -> f64 {
        self.quantity
    }

    pub(crate) fn get_total_in_cents(&self) -> i32 {
        self.unit_price_in_cents * self.quantity as i32
    }

    pub(crate) fn get_invoice_id(&self) -> &str {
        &self.invoice_id
    }
}

impl From<NewLineItem> for LineItem {
    fn from(value: NewLineItem) -> Self {
        let unit_price_in_cents = (value.get_unit_price() * 100.0) as i32;

        Self {
            id: generate_new_id(),
            description: value.get_description().into(),
            unit_price_in_cents,
            quantity: value.get_quantity(),
            invoice_id: value.get_invoice_id().into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::NewLineItem;

    #[test]
    fn test_from_new_line_item() {
        let new_line_item = NewLineItem::new("Test Item".into(), 10.0, 100.0, "1234567890".into());

        let line_item = LineItem::from(new_line_item);

        assert_eq!(line_item.description, "Test Item");
        assert_eq!(line_item.unit_price_in_cents, 10000);
        assert_eq!(line_item.quantity, 10.0);
        assert_eq!(line_item.id.len(), 10);
        assert_eq!(line_item.invoice_id, "1234567890");
    }
}
