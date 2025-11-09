use clap::Subcommand;

pub mod account;
pub mod client;
pub mod init;
pub mod invoice;
pub mod journal;

pub trait CommandHandler<C>
where
    C: Subcommand,
{
    async fn handle_command(&self, command: C);
}
