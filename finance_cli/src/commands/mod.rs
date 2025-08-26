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
    #[command(subcommand)]
    Client(ClientSubcommands),

    #[command(subcommand)]
    Invoice(InvoiceSubCommands),
}
