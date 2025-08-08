use clap::Args;

#[derive(Args)]
pub struct NewLineItem {
    name: String,
    quantity: f32,
    price: f32,
}

impl NewLineItem {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_quantity(&self) -> f32 {
        self.quantity
    }

    pub fn get_price(&self) -> f32 {
        self.price
    }

    pub fn get_total(&self) -> f32 {
        self.price * self.quantity
    }
}
