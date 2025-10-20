use clap::Subcommand;
use double_entry_bookkeeping::model::NewAccount;

#[derive(Subcommand)]
pub enum AccountSubcommands {
    #[command(visible_alias = "ls")]
    List,
    Get {
        account_id: u64,
    },
    Add {
        #[command(flatten)]
        new_account: NewAccount,
    },
}
