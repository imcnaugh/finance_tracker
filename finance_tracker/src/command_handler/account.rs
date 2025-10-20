use crate::command::account::AccountSubcommands;
use crate::config_service::get_config;
use crate::database::DatabaseManager;
use crate::sqlite_dao::account_sqlite_dao::AccountSqliteDao;
use crate::sqlite_dao::journal_sqlite_dao::JournalSqliteDao;
use double_entry_bookkeeping::service::account_service::AccountService;
use double_entry_bookkeeping::service::journal_service::JournalService;

pub struct AccountCommandHandler {
    account_service: AccountService<AccountSqliteDao>,
    journal_service: JournalService<JournalSqliteDao>,
}

impl AccountCommandHandler {
    pub async fn build() -> Result<Self, String> {
        let configuration =
            get_config().map_err(|_| "Configurations are not set, please run init")?;
        let db_configs = configuration.get_database_configuration();
        let db_manager = DatabaseManager::new(db_configs).await?;

        let account_dao = AccountSqliteDao::new(db_manager.get_pool().clone());
        let account_service = AccountService::new(account_dao);

        let journal_dao = JournalSqliteDao::new(db_manager.get_pool().clone());
        let journal_service = JournalService::new(journal_dao);

        Ok(Self {
            account_service,
            journal_service,
        })
    }

    pub async fn handle_account_command(&self, account_command: AccountSubcommands) {
        match account_command {
            AccountSubcommands::List => match self.account_service.get_all_accounts().await {
                Ok(accounts) => println!("{:?}", accounts),
                Err(e) => println!("Error: {}", e),
            },
            AccountSubcommands::Get { account_id } => {
                match self.account_service.get_account_by_id(account_id).await {
                    Ok(account) => {
                        let account = account
                            .ok_or_else(|| "Account not found".to_string())
                            .unwrap();
                        let balance = self
                            .journal_service
                            .get_account_balance(account.get_id())
                            .await
                            .unwrap();
                        println!("Account: {:?}\nBalance: {}", account, balance);
                    }
                    Err(e) => println!("Error: {}", e),
                }
            }
            AccountSubcommands::Add { new_account } => {
                todo!()
            }
        }
    }
}
