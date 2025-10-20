use crate::util::line_item_display::display_line_items;
use chrono::{DateTime, Utc};
use comfy_table::{Table, presets::UTF8_FULL};
use utilities::Error;
use invoice_manager::model::{Invoice, InvoiceStatus};

pub struct InvoiceDisplay {
    id: String,
    client_id: String,
    status: String,
    draft_date: String,
    sent_date: String,
    due_date: String,
    paid_date: String,
    cancelled_date: String,
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
    let id = InvoiceDisplay::from(invoice);
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);

    table.add_row(vec!["ID", id.id.as_str()]);
    table.add_row(vec!["Client ID", id.client_id.as_str()]);
    table.add_row(vec!["Status", id.status.as_str()]);
    table.add_row(vec!["Drafted", id.draft_date.as_str()]);
    table.add_row(vec!["Sent", id.sent_date.as_str()]);
    table.add_row(vec!["Due", id.due_date.as_str()]);
    table.add_row(vec!["Paid", id.paid_date.as_str()]);
    table.add_row(vec!["Cancelled", id.cancelled_date.as_str()]);
    table.add_row(vec!["Total", id.total.as_str()]);

    println!("Invoice:");
    println!("{}", table);

    display_line_items(invoice.get_line_items());
}

pub fn display_invoices(invoices: &[Invoice]) {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec!["ID", "Client ID", "Status", "Date", "Total"]);

    for inv in invoices {
        let id = InvoiceDisplay::from(inv);
        let date = match inv.get_status().unwrap() {
            InvoiceStatus::DRAFT => id.draft_date,
            InvoiceStatus::SENT => id.sent_date,
            InvoiceStatus::PAID => id.paid_date,
            InvoiceStatus::OVERDUE => id.due_date,
            InvoiceStatus::CANCELLED => id.cancelled_date,
        };
        table.add_row(vec![id.id, id.client_id, id.status, date, id.total]);
    }

    println!("Invoices:");
    if table.is_empty() {
        println!("No invoices found");
    } else {
        println!("{}", table);
    }
}

fn date_result_option_to_string(date: Result<Option<DateTime<Utc>>, Error>) -> String {
    date.ok()
        .flatten()
        .map(|d| d.format("%Y-%m-%d").to_string())
        .unwrap_or_default()
}

fn date_result_to_string(date: Result<DateTime<Utc>, Error>) -> String {
    date.ok()
        .map(|d| d.format("%Y-%m-%d").to_string())
        .unwrap_or_default()
}
