use crate::command::{Command, Commands};
use crate::command_handler::init::handle_init_command;
use crate::context::Context;
use clap::Parser;

mod command;
mod command_handler;
mod config_service;
mod configuration;
mod context;
mod database;
mod sqlite_dao;
mod util;

#[tokio::main]
async fn main() {
    let context = build_context().await.unwrap();

    match Command::parse().command {
        Commands::Client(client_command) => match context.get_client_command_handler() {
            Ok(handler) => handler.handle_client_command(client_command).await,
            Err(e) => println!("Error processing command: {}", e),
        },
        Commands::Invoice(invoice_command) => match context.get_invoice_command_handler() {
            Ok(handler) => handler.handle_invoice_command(invoice_command).await,
            Err(e) => println!("Error processing command: {}", e),
        },
        Commands::Account(account_command) => match context.get_account_command_handler() {
            Ok(handler) => handler.handle_account_command(account_command).await,
            Err(e) => println!("Error processing command: {}", e),
        },
        Commands::Journal(journal_command) => match context.get_journal_command_handler() {
            Ok(handler) => handler.handle_journal_command(journal_command).await,
            Err(e) => println!("Error processing command: {}", e),
        },
        Commands::Init(init_command) => handle_init_command(init_command).await,
    }
}

async fn build_context() -> Result<Context, String> {
    Ok(Context::new().await)
}
