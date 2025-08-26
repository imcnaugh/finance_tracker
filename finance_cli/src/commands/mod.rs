pub mod client;
pub mod invoice;

use crate::commands::client::ClientSubcommands;
use crate::commands::invoice::InvoiceSubCommands;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Command {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Client commands
    #[command(subcommand)]
    Client(ClientSubcommands),

    /// Invoice commands
    #[command(subcommand)]
    Invoice(InvoiceSubCommands),
}
