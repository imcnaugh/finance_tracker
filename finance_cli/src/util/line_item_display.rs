use invoice_manager::model::line_item::LineItem;
use tabled::settings::Style;
use tabled::{Table, Tabled};

#[derive(Tabled)]
pub struct LineItemDisplay {
    #[tabled(rename = "ID", order = 0)]
    id: String,

    #[tabled(rename = "Description", order = 1)]
    description: String,

    #[tabled(rename = "Quantity", order = 2)]
    quantity: f64,

    #[tabled(rename = "Unit Price", order = 3)]
    unit_price: String,

    #[tabled(rename = "Total", order = 4)]
    total: String,
}

impl From<&LineItem> for LineItemDisplay {
    fn from(value: &LineItem) -> Self {
        let display_value = format!("${:.2}", value.get_unit_price_in_cents() as f64 / 100.0);
        let display_total = format!("${:.2}", value.get_total_in_cents() as f64 / 100.0);
        Self {
            id: value.get_id().into(),
            description: value.get_description().into(),
            quantity: value.get_quantity(),
            unit_price: display_value,
            total: display_total,
        }
    }
}

pub fn display_line_items(line_items: &Vec<LineItem>) {
    let display_line_items: Vec<LineItemDisplay> =
        line_items.iter().map(LineItemDisplay::from).collect();
    let mut table = Table::new(&display_line_items);
    table.with(Style::modern());
    println!("{}", table);
}
