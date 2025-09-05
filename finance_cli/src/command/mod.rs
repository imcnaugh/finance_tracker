pub mod client;
pub mod init;
pub mod invoice;

use crate::command::client::ClientSubcommands;
use crate::command::init::InitCommand;
use crate::command::invoice::InvoiceSubCommands;
use clap::{Parser, Subcommand};

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
    Init(InitCommand),
}
