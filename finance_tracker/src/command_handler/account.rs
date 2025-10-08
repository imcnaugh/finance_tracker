use crate::command::account::AccountSubcommands;
use crate::config_service::get_config;
use crate::database::DatabaseManager;
use double_entry_bookkeeping::dao::sqlite::account_sqlite_dao::AccountSqliteDao;
use double_entry_bookkeeping::service::account_service::AccountService;

pub struct AccountCommandHandler {
    account_service: AccountService<AccountSqliteDao>,
}

impl AccountCommandHandler {
    pub async fn build() -> Result<Self, String> {
        let configuration =
            get_config().map_err(|_| "Configurations are not set, please run init")?;
        let db_configs = configuration.get_bookkeeping_database_configuration();
        let db_manager = DatabaseManager::new(db_configs).await?;
        let account_dao = AccountSqliteDao::new(db_manager.get_pool().clone());
        let account_service = AccountService::new(account_dao);
        Ok(Self { account_service })
    }

    pub async fn handle_account_command(&self, account_command: AccountSubcommands) {
        match account_command {
            AccountSubcommands::List => match self.account_service.get_all_account_types().await {
                Ok(accounts) => println!("{:?}", accounts),
                Err(e) => println!("Error: {}", e),
            },
        }
    }
}
