use clap::Subcommand;
use invoice_manager::model::NewClient;

#[derive(Subcommand)]
pub enum ClientSubcommands {
    /// Add a new client
    Add(NewClient),

    /// List all clients
    #[command(visible_alias = "ls")]
    List,
}
