use crate::command::invoice::InvoiceSubCommands;
use crate::database::DatabaseManager;
use crate::util;
use invoice_manager::dao::sqlite::client_sqlite_dao::ClientSqliteDao;
use invoice_manager::dao::sqlite::invoice_sqlite_dao::InvoiceSqliteDao;
use invoice_manager::model::Configuration;
use invoice_manager::service::{ClientService, InvoiceService, generate_pdf, get_config};
use utilities::prompt_confirm;

pub struct InvoiceCommandHandler {
    client_service: ClientService<ClientSqliteDao>,
    invoice_service: InvoiceService<InvoiceSqliteDao>,
    configuration: Configuration,
}

impl InvoiceCommandHandler {
    pub async fn build() -> Result<Self, String> {
        let configuration =
            get_config().map_err(|_| "Configurations are not set, please run init")?;
        let db_configs = configuration.get_database_configuration();
        let db_manager = DatabaseManager::new(db_configs).await?;

        let client_dao = ClientSqliteDao::new(db_manager.get_pool().clone());
        let invoice_dao = InvoiceSqliteDao::new(db_manager.get_pool().clone());

        let invoice_service = InvoiceService::new(Some(prompt_confirm), invoice_dao);
        let client_service = ClientService::new(client_dao);

        Ok(Self {
            client_service,
            invoice_service,
            configuration,
        })
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
