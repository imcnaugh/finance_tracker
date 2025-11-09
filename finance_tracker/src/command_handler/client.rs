use crate::command::client::ClientSubcommands;
use crate::sqlite_dao::client_sqlite_dao::ClientSqliteDao;
use crate::util;
use invoice_manager::service::ClientService;
use std::sync::Arc;

pub struct ClientCommandHandler {
    client_service: Arc<ClientService<ClientSqliteDao>>,
}

impl ClientCommandHandler {
    pub fn new(client_service: Arc<ClientService<ClientSqliteDao>>) -> Self {
        Self { client_service }
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
