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
    let command = Command::parse().command;

    if let Commands::Init(init_command) = command {
        handle_init_command(init_command).await;
        return;
    }

    let context = match Context::try_build().await {
        Ok(ctx) => ctx,
        Err(e) => {
            eprintln!("Failed to initialize application context: {}", e);
            return;
        }
    };

    match command {
        Commands::Client(client_command) => {
            context
                .get_client_command_handler()
                .handle_client_command(client_command)
                .await
        }
        Commands::Invoice(invoice_command) => {
            context
                .get_invoice_command_handler()
                .handle_invoice_command(invoice_command)
                .await
        }
        Commands::Account(account_command) => {
            context
                .get_account_command_handler()
                .handle_account_command(account_command)
                .await
        }
        Commands::Journal(journal_command) => {
            context
                .get_journal_command_handler()
                .handle_journal_command(journal_command)
                .await
        }
        Commands::Init(_) => panic!("This should be unreachable"),
    }
}
