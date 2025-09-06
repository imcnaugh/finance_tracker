pub mod client;

pub mod invoice;

use crate::command::client::ClientSubcommands;
use crate::command::invoice::InvoiceSubCommands;
use clap::{Parser, Subcommand};
use invoice_manager::model::NewConfig;

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

    /// Init
    Init(NewConfig),
}
