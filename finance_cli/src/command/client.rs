use clap::Subcommand;
use invoice_manager::model::NewClient;

#[derive(Subcommand)]
pub enum ClientSubcommands {
    /// Add a new client
    New {
        #[command(flatten)]
        new_client: NewClient,
    },

    /// Get a client by id
    Get { client_id: String },

    /// List all clients
    #[command(visible_alias = "ls")]
    List,
}
