use clap::Args;

#[derive(Args)]
pub struct NewLineItem {
    description: String,
    quantity: f64,
    unit_price: f64,
}

impl NewLineItem {
    pub(crate) fn new(description: String, quantity: f64, price: f64) -> Self {
        Self {
            description,
            quantity,
            unit_price: price,
        }
    }

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
