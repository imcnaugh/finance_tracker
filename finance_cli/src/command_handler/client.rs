use crate::command::client::ClientSubcommands;
use crate::util;
use invoice_manager::service::ClientService;

pub async fn handle_client_command(client_command: ClientSubcommands) {
    let service = ClientService::new();

    match client_command {
        ClientSubcommands::New(new_client) => {
            match service.create_client(new_client).await {
                Ok(client) => println!("Client created: {:?}", client),
                Err(e) => println!("Error creating client: {:?}", e),
            };
        }
        ClientSubcommands::List => match service.get_all_clients().await {
            Ok(clients) => util::client_display::display_clients(&clients),
            Err(e) => println!("Error getting clients: {:?}", e),
        },
    }
}
