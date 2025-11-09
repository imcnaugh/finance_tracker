use crate::command::invoice::InvoiceSubCommands;
use crate::configuration::Configuration;
use crate::sqlite_dao::client_sqlite_dao::ClientSqliteDao;
use crate::sqlite_dao::invoice_sqlite_dao::InvoiceSqliteDao;
use crate::util;
use invoice_manager::service::{ClientService, InvoiceService, generate_pdf};
use std::sync::Arc;

pub struct InvoiceCommandHandler {
    client_service: Arc<ClientService<ClientSqliteDao>>,
    invoice_service: Arc<InvoiceService<InvoiceSqliteDao>>,
    configuration: Arc<Configuration>,
}

impl InvoiceCommandHandler {
    pub fn new(
        client_service: Arc<ClientService<ClientSqliteDao>>,
        invoice_service: Arc<InvoiceService<InvoiceSqliteDao>>,
        configuration: Arc<Configuration>,
    ) -> Self {
        Self {
            client_service,
            invoice_service,
            configuration,
        }
    }

    pub async fn handle_invoice_command(&self, invoice_command: InvoiceSubCommands) {
        match invoice_command {
            InvoiceSubCommands::New { client_id } => {
                match self.invoice_service.create_new_invoice(client_id).await {
                    Ok(invoice) => util::invoice_display::display_invoice(&invoice),
                    Err(e) => println!("Error creating invoice: {:?}", e),
                }
            }
            InvoiceSubCommands::Get { invoice_id } => {
                match self.invoice_service.get_invoice(&invoice_id).await {
                    Ok(invoice) => util::invoice_display::display_invoice(&invoice),
                    Err(e) => println!("Error: {}", e.as_str()),
                }
            }
            InvoiceSubCommands::List { search_options } => {
                match self.invoice_service.search_invoices(search_options).await {
                    Ok(invoices) => util::invoice_display::display_invoices(&invoices),
                    Err(e) => println!("Error getting invoices: {:?}", e),
                };
            }
            InvoiceSubCommands::AddItem {
                invoice_id,
                new_line_item,
            } => {
                match self
                    .invoice_service
                    .add_line_item_to_invoice(&invoice_id, &new_line_item)
                    .await
                {
                    Ok(invoice) => util::invoice_display::display_invoice(&invoice),
                    Err(e) => println!("Error updating invoice: {:?}", e),
                }
            }
            InvoiceSubCommands::DeleteItem {
                invoice_id,
                line_item_id,
            } => {
                match self
                    .invoice_service
                    .delete_line_item_from_invoice(&invoice_id, &line_item_id)
                    .await
                {
                    Ok(invoice) => util::invoice_display::display_invoice(&invoice),
                    Err(e) => println!("Error updating invoice: {:?}", e),
                }
            }
            InvoiceSubCommands::Send {
                invoice_id,
                generate_pdf,
            } => match self.invoice_service.mark_invoice_sent(&invoice_id).await {
                Ok(invoice) => {
                    if generate_pdf {
                        let client = self
                            .client_service
                            .get_client_by_id(invoice.get_client_id())
                            .await
                            .unwrap();
                        invoice_manager::service::generate_pdf(
                            &invoice,
                            &client,
                            self.configuration.get_company_configuration(),
                        );
                    }
                    util::invoice_display::display_invoice(&invoice);
                }
                Err(e) => println!("Error sending invoice: {:?}", e),
            },
            InvoiceSubCommands::Paid { invoice_id } => {
                match self.invoice_service.mark_invoice_paid(&invoice_id).await {
                    Ok(invoice) => util::invoice_display::display_invoice(&invoice),
                    Err(e) => println!("Error marking invoice as paid: {:?}", e),
                }
            }
            InvoiceSubCommands::Cancel { invoice_id } => {
                match self
                    .invoice_service
                    .mark_invoice_cancelled(&invoice_id)
                    .await
                {
                    Ok(invoice) => util::invoice_display::display_invoice(&invoice),
                    Err(e) => println!("Error marking invoice as paid: {:?}", e),
                }
            }
            InvoiceSubCommands::GeneratePdf { invoice_id } => {
                match self.invoice_service.get_invoice(&invoice_id).await {
                    Ok(invoice) => {
                        match self
                            .client_service
                            .get_client_by_id(invoice.get_client_id())
                            .await
                        {
                            Ok(client) => {
                                generate_pdf(
                                    &invoice,
                                    &client,
                                    self.configuration.get_company_configuration(),
                                );
                            }
                            Err(e) => println!("Error getting client: {:?}", e),
                        }
                    }
                    Err(e) => println!("Error: {}", e.as_str()),
                }
            }
        }
    }
}
