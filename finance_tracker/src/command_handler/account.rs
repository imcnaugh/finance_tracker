use crate::command::account::AccountSubcommands;
use crate::command_handler::CommandHandler;
use crate::sqlite_dao::account_sqlite_dao::AccountSqliteDao;
use crate::util;
use double_entry_bookkeeping::account_service::AccountService;
use std::sync::Arc;

pub struct AccountCommandHandler {
    account_service: Arc<AccountService<AccountSqliteDao>>,
}

impl CommandHandler<AccountSubcommands> for AccountCommandHandler {
    async fn handle_command(&self, command: AccountSubcommands) {
        match command {
            AccountSubcommands::List => match self.account_service.get_all_accounts().await {
                Ok(accounts) => util::account_display::display_accounts(&accounts),
                Err(e) => println!("Error: {}", e),
            },
            AccountSubcommands::Get { account_id } => {
                match self.account_service.get_account_by_id(account_id).await {
                    Ok(account) => util::account_display::display_account(&account),
                    Err(e) => println!("Error: {}", e),
                }
            }
            AccountSubcommands::Add {
                new_account: _new_account,
            } => {
                todo!()
            }
        }
    }
}

impl AccountCommandHandler {
    pub fn new(account_service: Arc<AccountService<AccountSqliteDao>>) -> Self {
        Self { account_service }
    }
}
