use invoice_manager::model::client::Client;
use invoice_manager::model::invoice::Invoice;
use tabled::Tabled;

#[derive(Tabled)]
pub struct InvoiceDetails {
    id: String,
    client_id: String,
    status: String,
    total: String,
}

#[derive(Tabled)]
pub struct ClientDetails {
    id: String,
    name: String,
    address: String,
    phone: String,
    invoice_email: String,
}

impl From<&Invoice> for InvoiceDetails {
    fn from(value: &Invoice) -> Self {
        let total_in_cents = value.get_line_items().iter().fold(0.0, |acc, li| {
            acc + (li.get_quantity() * li.get_total_in_cents() as f64)
        });
        let total_in_cents = format!("${:.2}", total_in_cents / 100.0);
        Self {
            id: value.get_id().into(),
            client_id: value.get_client_id().into(),
            status: value.get_status().unwrap().to_string(),
            total: total_in_cents,
        }
    }
}

impl From<&Client> for ClientDetails {
    fn from(value: &Client) -> Self {
        Self {
            id: value.get_id().into(),
            name: value.get_name().into(),
            address: value.get_address().clone().unwrap_or_default(),
            phone: value.get_phone().clone().unwrap_or_default(),
            invoice_email: value.get_invoice_email().clone().unwrap_or_default(),
        }
    }
}
