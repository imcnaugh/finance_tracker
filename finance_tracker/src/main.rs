use crate::command::{Command, Commands};
use crate::command_handler::client::ClientCommandHandler;
use crate::command_handler::init::handle_init_command;
use crate::command_handler::invoice::InvoiceCommandHandler;
use clap::Parser;

mod command;
mod command_handler;
mod database;
mod util;

#[tokio::main]
async fn main() {
    match Command::parse().command {
        Commands::Client(client_command) => match ClientCommandHandler::build().await {
            Ok(handler) => handler.handle_client_command(client_command).await,
            Err(e) => println!("Error processing command: {}", e),
        },
        Commands::Invoice(invoice_command) => match InvoiceCommandHandler::build().await {
            Ok(handler) => handler.handle_invoice_command(invoice_command).await,
            Err(e) => println!("Error processing command: {}", e),
        },
        Commands::Init(init_command) => handle_init_command(init_command).await,
    }
}
