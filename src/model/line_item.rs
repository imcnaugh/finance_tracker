use crate::model::NewLineItem;
use crate::utils::generate_new_id;

pub(crate) struct LineItem {
    id: String,
    name: String,
    price_in_cents: isize,
    quantity: f32,
}

impl LineItem {}

impl From<NewLineItem> for LineItem {
    fn from(value: NewLineItem) -> Self {
        let price_in_cents = (value.get_price() * 100.0) as isize;

        Self {
            id: generate_new_id(),
            name: value.get_name().into(),
            price_in_cents,
            quantity: value.get_quantity(),
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

        let line_item = LineItem::from(new_line_item);

        assert_eq!(line_item.name, "Test Item");
        assert_eq!(line_item.price_in_cents, 10000);
        assert_eq!(line_item.quantity, 10.0);
        assert_eq!(line_item.id.len(), 10);
    }
}
