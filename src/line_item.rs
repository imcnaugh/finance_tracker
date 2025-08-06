use clap::Args;

#[derive(Args)]
pub struct LineItem {
    name: String,
    quantity: f32,
    #[arg(value_parser = price_parser)]
    price: isize,
}

fn price_parser(s: &str) -> Result<isize, String> {
    let float_price: f32 = s
        .parse()
        .map_err(|_| "Invalid price format, it should parse to a float")?;
    Ok((float_price * 100.0) as isize)
}

impl LineItem {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_quantity(&self) -> f32 {
        self.quantity
    }

    pub fn get_price(&self) -> f32 {
        self.price as f32 / 100.0
    }

    pub fn get_total(&self) -> f32 {
        self.get_price() * self.quantity
    }
}
