use crate::command::{Command, Commands};
use crate::command_handler::client::handle_client_command;
use crate::command_handler::init::handle_init_command;
use crate::command_handler::invoice::handle_invoice_command;
use clap::Parser;

mod command;
mod command_handler;
mod util;

#[tokio::main]
async fn main() {
    match Command::parse().command {
        Commands::Client(client_command) => handle_client_command(client_command).await,
        Commands::Invoice(invoice_command) => handle_invoice_command(invoice_command).await,
        Commands::Init(init_command) => handle_init_command(init_command).await,
    }
}
