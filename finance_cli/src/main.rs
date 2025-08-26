use crate::commands::client::ClientSubcommands;
use crate::commands::invoice::InvoiceSubCommands;
use crate::commands::{Command, Commands};
use crate::tableds::{ClientDetails, InvoiceDetails};
use clap::Parser;
use invoice_manager::service::{ClientService, InvoiceService};
use tabled::Table;
use tabled::settings::Style;

mod commands;
pub mod tableds;

#[tokio::main]
async fn main() {
    let cli = Command::parse();
    match cli.command {
        Commands::Client(client_command) => {
            let service = ClientService::new();

            match client_command {
                ClientSubcommands::Add(new_client) => {
                    match service.create_client(new_client).await {
                        Ok(client) => println!("Client created: {:?}", client),
                        Err(e) => println!("Error creating client: {:?}", e),
                    };
                }
                ClientSubcommands::List => match service.get_all_clients().await {
                    Ok(clients) => {
                        println!("Clients:");
                        let clients: Vec<ClientDetails> =
                            clients.iter().map(ClientDetails::from).collect();
                        let mut tabled_clients = Table::new(clients);
                        tabled_clients.with(Style::psql());
                        println!("{}", tabled_clients);
                    }
                    Err(e) => println!("Error getting clients: {:?}", e),
                },
            }
        }
        Commands::Invoice(invoice_command) => match invoice_command {
            InvoiceSubCommands::New { client_id } => {
                let invoice_service = InvoiceService::new();
                match invoice_service.create_new_invoice(client_id).await {
                    Ok(invoice) => println!("Invoice created: {:?}", invoice),
                    Err(e) => println!("Error creating invoice: {:?}", e),
                }
            }
            InvoiceSubCommands::List { search_options } => {
                let invoice_service = InvoiceService::new();

                match invoice_service.search_invoices(search_options).await {
                    Ok(invoices) => {
                        println!("Invoices:");
                        let invoices: Vec<InvoiceDetails> =
                            invoices.iter().map(InvoiceDetails::from).collect();
                        let mut tabled_invoices = Table::new(invoices);
                        tabled_invoices.with(Style::psql());
                        println!("{}", tabled_invoices);
                    }
                    Err(e) => println!("Error getting invoices: {:?}", e),
                };
            }

            _ => {}
        },
    }
}
