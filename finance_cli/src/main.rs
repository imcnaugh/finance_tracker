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
        Commands::Invoice(invoice_command) => {
            let invoice_service = InvoiceService::new();
            match invoice_command {
                InvoiceSubCommands::New { client_id } => {
                    match invoice_service.create_new_invoice(client_id).await {
                        Ok(invoice) => println!("Invoice created: {:?}", invoice),
                        Err(e) => println!("Error creating invoice: {:?}", e),
                    }
                }
                InvoiceSubCommands::Get { invoice_id } => {
                    match invoice_service.get_invoice(&invoice_id).await {
                        Ok(invoice) => {
                            let mut tabled_invoices = Table::new([InvoiceDetails::from(&invoice)]);
                            tabled_invoices.with(Style::psql());
                            println!("{}", tabled_invoices);
                        }
                        Err(e) => println!("Error: {}", e.as_str()),
                    }
                }
                InvoiceSubCommands::List { search_options } => {
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
                InvoiceSubCommands::AddItem {
                    invoice_id,
                    new_line_item,
                } => {
                    match invoice_service
                        .add_line_item_to_invoice(&invoice_id, &new_line_item)
                        .await
                    {
                        Ok(invoice) => println!("Invoice updated: {:?}", invoice),
                        Err(e) => println!("Error updating invoice: {:?}", e),
                    }
                }
                InvoiceSubCommands::DeleteItem {
                    invoice_id,
                    line_item_id,
                } => {}
                InvoiceSubCommands::UpdateStatus {
                    invoice_id,
                    status,
                    generate_pdf,
                } => {
                    match invoice_service
                        .update_invoice_status(&invoice_id, &status)
                        .await
                    {
                        Ok(_) => {
                            println!("Invoice status updated");
                            //TODO generate pdf
                        }
                        Err(e) => println!("Error updating invoice status: {:?}", e),
                    }
                }
                InvoiceSubCommands::GeneratePdf { invoice_id } => {}
            }
        }
    }
}
