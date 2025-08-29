use crate::util::line_item_display::display_line_items;
use chrono::{DateTime, Utc};
use invoice_manager::model::invoice::Invoice;
use tabled::settings::object::Columns;
use tabled::settings::{Remove, Rotate, Style};
use tabled::{Table, Tabled};

#[derive(Tabled)]
pub struct InvoiceDisplay {
    #[tabled(rename = "ID", order = 0)]
    id: String,

    #[tabled(rename = "Client ID", order = 1)]
    client_id: String,

    #[tabled(rename = "Status", order = 2)]
    status: String,

    #[tabled(rename = "Drafted", order = 3)]
    draft_date: String,

    #[tabled(rename = "Sent", order = 4)]
    sent_date: String,

    #[tabled(rename = "Due", order = 5)]
    due_date: String,

    #[tabled(rename = "Paid", order = 6)]
    paid_date: String,

    #[tabled(rename = "Cancelled", order = 7)]
    cancelled_date: String,

    #[tabled(rename = "Total", order = 8)]
    total: String,
}

impl From<&Invoice> for InvoiceDisplay {
    fn from(value: &Invoice) -> Self {
        let total_in_cents = value
            .get_line_items()
            .iter()
            .fold(0i64, |acc, li| acc + li.get_total_in_cents() as i64);
        let total_in_cents = format!("${:.2}", total_in_cents as f64 / 100.0);

        Self {
            id: value.get_id().into(),
            client_id: value.get_client_id().into(),
            status: value.get_status().unwrap().to_string(),
            draft_date: date_result_to_string(value.get_draft_date()),
            sent_date: date_result_option_to_string(value.get_sent_date()),
            due_date: date_result_option_to_string(value.get_due_date()),
            paid_date: date_result_option_to_string(value.get_paid_date()),
            cancelled_date: date_result_option_to_string(value.get_cancelled_date()),
            total: total_in_cents,
        }
    }
}

pub fn display_invoice(invoice: &Invoice) {
    let invoice_display = InvoiceDisplay::from(invoice);
    let mut table = Table::new([&invoice_display]);
    table
        .with(Rotate::Left)
        .with(Rotate::Bottom)
        .with(Style::modern().remove_horizontal());
    println!("{}", table);

    display_line_items(invoice.get_line_items());
}

pub fn display_invoices(invoices: &[Invoice]) {
    let invoice_displays = invoices
        .iter()
        .map(|i| InvoiceDisplay::from(i))
        .collect::<Vec<_>>();
    let mut table = Table::new(invoice_displays);
    table
        .with(Remove::column(Columns::new(3..=7)))
        .with(Style::modern());
    println!("{}", table);
}

fn date_result_option_to_string(date: Result<Option<DateTime<Utc>>, ()>) -> String {
    date.ok()
        .flatten()
        .map(|d| d.format("%Y-%m-%d").to_string())
        .unwrap_or_default()
}

fn date_result_to_string(date: Result<DateTime<Utc>, ()>) -> String {
    date.ok()
        .map(|d| d.format("%Y-%m-%d").to_string())
        .unwrap_or_default()
}
