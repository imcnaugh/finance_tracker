use crate::command::init::InitCommand;

pub async fn handle_init_command(init_command: InitCommand) {
    invoice_manager::service::create_config(init_command);
}
