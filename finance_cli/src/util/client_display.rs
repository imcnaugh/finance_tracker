use invoice_manager::model::client::Client;
use comfy_table::{presets::UTF8_FULL, Table};

struct ClientDisplay {
    id: String,
    name: String,
    address: String,
    phone: String,
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
    let cd = ClientDisplay::from(client);
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);

    table.add_row(vec!["ID", cd.id.as_str()]);
    table.add_row(vec!["Name", cd.name.as_str()]);
    table.add_row(vec!["Address", cd.address.as_str()]);
    table.add_row(vec!["Phone", cd.phone.as_str()]);
    table.add_row(vec!["Invoice Email", cd.invoice_email.as_str()]);

    println!("{}", table);
}

pub fn display_clients(clients: &Vec<Client>) {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec!["ID", "Name", "Address", "Phone", "Invoice Email"]);

    for client in clients {
        let cd = ClientDisplay::from(client);
        table.add_row(vec![
            cd.id,
            cd.name,
            cd.address,
            cd.phone,
            cd.invoice_email,
        ]);
    }

    println!("{}", table);
}
