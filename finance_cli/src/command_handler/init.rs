use invoice_manager::model::NewCompanyConfig;

pub async fn handle_init_command(new_config: NewCompanyConfig) {
    match invoice_manager::service::create_config(new_config) {
        Ok(_) => println!("Config created successfully"),
        Err(e) => println!("Error creating config: {:?}", e),
    }
}
