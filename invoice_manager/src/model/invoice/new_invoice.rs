use clap::Args;

#[derive(Args)]
pub struct NewInvoice {
    client_id: String,
}

impl NewInvoice {
    pub fn new(client_id: String) -> Self {
        Self { client_id }
    }

    pub fn get_client_id(&self) -> &String {
        &self.client_id
    }
}
