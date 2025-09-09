use crate::command::client::ClientSubcommands;
use crate::util;
use invoice_manager::dao::sqlite::client_sqlite_dao::ClientSqliteDao;
use invoice_manager::dao::sqlite::sqlite_connection::get_pooled_connection;
use invoice_manager::service::{ClientService, get_config};
use std::sync::Arc;

pub async fn handle_client_command(client_command: ClientSubcommands) {
    let configuration = get_config().unwrap();
    let db_configs = configuration.get_database_configuration().unwrap();
    let pool = get_pooled_connection(db_configs).await;
    let client_dao = ClientSqliteDao::new(pool);
    let service = ClientService::new(client_dao);

    match client_command {
        ClientSubcommands::New { new_client } => {
            match service.create_client(new_client).await {
                Ok(client) => util::client_display::display_client(&client),
                Err(e) => println!("Error creating client: {:?}", e),
            };
        }
        ClientSubcommands::Get { client_id } => match service.get_client_by_id(&client_id).await {
            Ok(client) => util::client_display::display_client(&client),
            Err(e) => println!("Error getting client: {:?}", e),
        },
        ClientSubcommands::List => match service.get_all_clients().await {
            Ok(clients) => util::client_display::display_clients(&clients),
            Err(e) => println!("Error getting clients: {:?}", e),
        },
    }
}
