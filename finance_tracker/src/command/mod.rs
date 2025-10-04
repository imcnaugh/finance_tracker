pub mod client;

pub mod account;
pub mod invoice;

use crate::command::account::AccountSubcommands;
use crate::command::client::ClientSubcommands;
use crate::command::invoice::InvoiceSubCommands;
use clap::{Parser, Subcommand};
use invoice_manager::model::NewCompanyConfiguration;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Command {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Client command
    #[command(subcommand)]
    Client(ClientSubcommands),

    /// Invoice command
    #[command(subcommand)]
    Invoice(InvoiceSubCommands),

    /// Account command
    #[command(subcommand)]
    Account(AccountSubcommands),

    /// Init
    Init(NewCompanyConfiguration),
}
