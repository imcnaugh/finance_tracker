use invoice_manager::model::client::Client;
use tabled::settings::{Rotate, Style};
use tabled::{Table, Tabled};

#[derive(Tabled)]
struct ClientDisplay {
    #[tabled(rename = "ID", order = 0)]
    id: String,

    #[tabled(rename = "Name", order = 1)]
    name: String,

    #[tabled(rename = "Address", order = 2)]
    address: String,

    #[tabled(rename = "Phone", order = 3)]
    phone: String,

    #[tabled(rename = "Invoice Email", order = 4)]
    invoice_email: String,
}

impl From<&Client> for ClientDisplay {
    fn from(value: &Client) -> Self {
        Self {
            id: value.get_id().into(),
            name: value.get_name().into(),
            address: value.get_address().unwrap_or_default().into(),
            phone: value.get_phone().unwrap_or_default().into(),
            invoice_email: value.get_invoice_email().unwrap_or_default().into(),
        }
    }
}

pub fn display_client(client: &Client) {
    let client_display = ClientDisplay::from(client);
    let mut table = Table::new([&client_display]);
    table
        .with(Rotate::Left)
        .with(Rotate::Bottom)
        .with(Style::modern().remove_horizontal());
    println!("{}", table);
}

pub fn display_clients(clients: &Vec<Client>) {
    let client_displays: Vec<ClientDisplay> = clients
        .iter()
        .map(|client| ClientDisplay::from(client))
        .collect();
    let mut table = Table::new(&client_displays);
    table.with(Style::modern());
    println!("{}", table);
}
