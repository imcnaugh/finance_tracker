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
