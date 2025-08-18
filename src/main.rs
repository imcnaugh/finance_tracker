use clap::{Args, Parser, Subcommand};
use invoice_generator::model::{NewClient, NewLineItem};
use invoice_generator::service::{ClientService, LineItemService};

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
                    Ok(clients) => println!("Clients: {:?}", clients),
                    Err(e) => println!("Error getting clients: {:?}", e),
                },
            }
        }
        Commands::Invoice(invoice_command) => match invoice_command {
            InvoiceSubCommands::New { client_id } => {
                println!("Creating new invoice for client: {}", client_id);
            }
            InvoiceSubCommands::List { status, client_id } => {
                println!("Listing invoices");
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
