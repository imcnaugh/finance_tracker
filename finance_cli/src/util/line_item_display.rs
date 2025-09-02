use comfy_table::{Table, presets::UTF8_FULL};
use invoice_manager::model::line_item::LineItem;

pub struct LineItemDisplay {
    id: String,
    description: String,
    quantity: f64,
    unit_price: String,
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

pub fn display_line_items(line_items: &[LineItem]) {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec!["ID", "Description", "Quantity", "Unit Price", "Total"]);

    for li in line_items.iter().map(LineItemDisplay::from) {
        table.add_row(vec![
            li.id,
            li.description,
            format!("{}", li.quantity),
            li.unit_price,
            li.total,
        ]);
    }

    println!("Line Items:");
    if table.is_empty() {
        println!("No line items found");
    } else {
        println!("{}", table);
    }
}
