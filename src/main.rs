use clap::{Args, Parser, Subcommand};
use invoice_generator::model::invoice::Invoice;
use invoice_generator::model::{NewClient, NewLineItem};
use invoice_generator::service::{ClientService, InvoiceService};

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
                        clients
                            .iter()
                            .for_each(|client| println!("Client: {:?}", client));
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
            InvoiceSubCommands::List { status, client_id } => {
                let invoice_service = InvoiceService::new();
                match invoice_service.list_invoices().await {
                    Ok(invoices) => {
                        invoices
                            .iter()
                            .for_each(|invoice| println!("Invoice: {:?}", invoice));
                    }
                    Err(e) => println!("Error getting invoices: {:?}", e),
                };
            }
        },
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Command {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(subcommand)]
    Client(ClientSubcommands),

    #[command(subcommand)]
    Invoice(InvoiceSubCommands),
}

#[derive(Subcommand)]
enum ClientSubcommands {
    Add(NewClient),

    #[command(alias = "ls")]
    List,
}

#[derive(Subcommand)]
enum InvoiceSubCommands {
    New {
        #[clap(short, long)]
        client_id: String,
    },
    List {
        #[clap(short, long)]
        status: Option<String>,

        #[clap(short, long)]
        client_id: Option<String>,
    },
}
