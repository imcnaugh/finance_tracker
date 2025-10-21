use clap::Args;

#[derive(Args)]
pub struct NewLineItem {
    description: String,
    quantity: f64,
    unit_price: f64,
}

impl NewLineItem {
    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_quantity(&self) -> f64 {
        self.quantity
    }

    pub fn get_unit_price(&self) -> f64 {
        self.unit_price
    }
}

#[cfg(test)]
mod tests {
    use crate::model::NewLineItem;
    use crate::model::line_item::LineItem;

    #[test]
    fn test_from_new_line_item() {
        let new_line_item = NewLineItem {
            description: "Test Item".into(),
            quantity: 10.0,
            unit_price: 100.0,
        };

        let line_item = LineItem::from((&new_line_item, "1234567890"));

        assert_eq!(line_item.get_description(), "Test Item");
        assert_eq!(line_item.get_total_in_cents(), 100000);
        assert_eq!(line_item.get_quantity(), 10.0);
        assert_eq!(line_item.get_id().len(), 10);
        assert_eq!(line_item.get_invoice_id(), "1234567890");
    }
}
