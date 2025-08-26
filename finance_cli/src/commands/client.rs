use clap::Subcommand;
use invoice_manager::model::NewClient;

#[derive(Subcommand)]
pub enum ClientSubcommands {
    Add(NewClient),

    #[command(alias = "ls")]
    List,
}
