use crate::command::invoice::InvoiceSubCommands;
use crate::util;
use invoice_manager::service::{generate_pdf, ClientService, InvoiceService};

pub async fn handle_invoice_command(invoice_command: InvoiceSubCommands) {
    let invoice_service = InvoiceService::new(Some(util::prompt_confirm));
    let client_service = ClientService::new();

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
            match invoice_service.get_invoice(&invoice_id).await {
                Ok(invoice) => {
                    util::invoice_display::display_invoice(&invoice);

                    match invoice_service.mark_invoice_sent(&invoice_id).await {
                        Ok(invoice) => {
                            // TODO generate pdf
                            println!("Invoice sent: {:?}", invoice);
                        }
                        Err(e) => println!("Error sending invoice: {:?}", e),
                    }
                }
                Err(e) => println!("Error: {}", e.as_str()),
            }
        }
        InvoiceSubCommands::Paid { invoice_id } => {
            match invoice_service.get_invoice(&invoice_id).await {
                Ok(invoice) => {
                    util::invoice_display::display_invoice(&invoice);

                    match invoice_service.mark_invoice_paid(&invoice_id).await {
                        Ok(invoice) => println!("Invoice marked as paid: {:?}", invoice),
                        Err(e) => println!("Error marking invoice as paid: {:?}", e),
                    }
                }
                Err(e) => println!("Error: {}", e.as_str()),
            }
        }
        InvoiceSubCommands::Cancel { invoice_id } => {
            match invoice_service.get_invoice(&invoice_id).await {
                Ok(invoice) => {
                    util::invoice_display::display_invoice(&invoice);

                    match invoice_service.mark_invoice_cancelled(&invoice_id).await {
                        Ok(invoice) => println!("Invoice marked as cancelled: {:?}", invoice),
                        Err(e) => println!("Error marking invoice as cancelled: {:?}", e),
                    }
                }
                Err(e) => println!("Error: {}", e.as_str()),
            }
        }
        InvoiceSubCommands::GeneratePdf { invoice_id } => {
            match invoice_service.get_invoice(&invoice_id).await {
                Ok(invoice) => {
                    match client_service.get_client_by_id(invoice.get_client_id()).await {
                        Ok(client) => {
                            generate_pdf(&invoice, &client);
                        }
                        Err(e) => println!("Error getting client: {:?}", e),
                    }
                }
                Err(e) => println!("Error: {}", e.as_str()),
            }
        }
    }
}
