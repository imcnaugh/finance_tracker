use crate::config_service::create_config;
use invoice_manager::model::NewCompanyConfiguration;

pub async fn handle_init_command(new_config: NewCompanyConfiguration) {
    match create_config(new_config) {
        Ok(_) => println!("Config created successfully"),
        Err(e) => println!("Error creating config: {:?}", e),
    }
}
