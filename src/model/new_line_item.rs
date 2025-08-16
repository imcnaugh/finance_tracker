use clap::Args;

#[derive(Args)]
pub struct NewLineItem {
    description: String,
    quantity: f32,
    unit_price: f32,
}

impl NewLineItem {
    pub(crate) fn new(description: String, quantity: f32, price: f32) -> Self {
        Self {
            description,
            quantity,
            unit_price: price,
        }
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_quantity(&self) -> f32 {
        self.quantity
    }

    pub fn get_unit_price(&self) -> f32 {
        self.unit_price
    }
}
