use crate::command::client::ClientSubcommands;
use crate::config_service::get_config;
use crate::database::DatabaseManager;
use crate::util;
use invoice_manager::dao::sqlite::client_sqlite_dao::ClientSqliteDao;
use invoice_manager::service::ClientService;

pub struct ClientCommandHandler {
    client_service: ClientService<ClientSqliteDao>,
}

impl ClientCommandHandler {
    pub async fn build() -> Result<Self, String> {
        let configuration =
            get_config().map_err(|_| "Configurations are not set, please run init")?;
        let db_configs = configuration.get_database_configuration();
        let db_manager = DatabaseManager::new(db_configs).await?;
        let client_dao = ClientSqliteDao::new(db_manager.get_pool().clone());
        let client_service = ClientService::new(client_dao);
        Ok(Self { client_service })
    }

    pub async fn handle_client_command(&self, client_command: ClientSubcommands) {
        match client_command {
            ClientSubcommands::New { new_client } => {
                match self.client_service.create_client(new_client).await {
                    Ok(client) => util::client_display::display_client(&client),
                    Err(e) => println!("Error creating client: {:?}", e),
                };
            }
            ClientSubcommands::Get { client_id } => {
                match self.client_service.get_client_by_id(&client_id).await {
                    Ok(client) => util::client_display::display_client(&client),
                    Err(e) => println!("Error getting client: {:?}", e),
                }
            }
            ClientSubcommands::List => match self.client_service.get_all_clients().await {
                Ok(clients) => util::client_display::display_clients(&clients),
                Err(e) => println!("Error getting clients: {:?}", e),
            },
        }
    }
}
