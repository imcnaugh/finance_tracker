use clap::Args;

#[derive(Args)]
pub struct NewLineItem {
    description: String,
    quantity: f32,
    unit_price: f32,
    invoice_id: String,
}

impl NewLineItem {
    pub(crate) fn new(description: String, quantity: f32, price: f32, invoice_id: String) -> Self {
        Self {
            description,
            quantity,
            unit_price: price,
            invoice_id,
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

    pub fn get_invoice_id(&self) -> &str {
        &self.invoice_id
    }
}
