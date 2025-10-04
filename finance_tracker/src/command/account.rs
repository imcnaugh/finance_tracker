use clap::Subcommand;

#[derive(Subcommand)]
pub enum AccountSubcommands {
    #[command(visible_alias = "ls")]
    List,
}
