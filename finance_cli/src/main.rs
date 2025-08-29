use crate::command::client::ClientSubcommands;
use crate::command::invoice::InvoiceSubCommands;
use crate::command::{Command, Commands};
use clap::Parser;
use invoice_manager::service::{ClientService, InvoiceService};

mod command;
mod util;

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
                    Ok(clients) => util::client_display::display_clients(&clients),
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
                            util::invoice_display::display_invoice(&invoice);
                        }
                        Err(e) => println!("Error: {}", e.as_str()),
                    }
                }
                InvoiceSubCommands::List { search_options } => {
                    match invoice_service.search_invoices(search_options).await {
                        Ok(invoices) => {
                            util::invoice_display::display_invoices(&invoices);
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
                } => {
                    match invoice_service
                        .delete_line_item_from_invoice(&invoice_id, &line_item_id)
                        .await
                    {
                        Ok(invoice) => println!("Invoice updated: {:?}", invoice),
                        Err(e) => println!("Error updating invoice: {:?}", e),
                    }
                }
                InvoiceSubCommands::Send {
                    invoice_id,
                    generate_pdf,
                } => {
                    // TODO display invoice and confirm send
                    match invoice_service.mark_invoice_sent(&invoice_id).await {
                        Ok(invoice) => {
                            // TODO generate pdf
                            println!("Invoice sent: {:?}", invoice);
                        }
                        Err(e) => println!("Error sending invoice: {:?}", e),
                    }
                }
                InvoiceSubCommands::Paid { invoice_id } => {
                    // TODO display invoice and confirm paid
                    match invoice_service.mark_invoice_paid(&invoice_id).await {
                        Ok(invoice) => println!("Invoice marked as paid: {:?}", invoice),
                        Err(e) => println!("Error marking invoice as paid: {:?}", e),
                    }
                }
                InvoiceSubCommands::Cancel { invoice_id } => {
                    // TODO display invoice and confirm cancel
                    match invoice_service.mark_invoice_cancelled(&invoice_id).await {
                        Ok(invoice) => println!("Invoice marked as cancelled: {:?}", invoice),
                        Err(e) => println!("Error marking invoice as cancelled: {:?}", e),
                    }
                }
                InvoiceSubCommands::GeneratePdf { invoice_id } => {
                    todo!()
                }
            }
        }
    }
}
