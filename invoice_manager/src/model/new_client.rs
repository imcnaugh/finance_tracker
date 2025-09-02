use clap::Args;

#[derive(Args)]
pub struct NewClient {
    name: String,

    #[arg(long)]
    address: Option<String>,

    #[arg(long)]
    phone: Option<String>,

    #[arg(long)]
    invoice_email: Option<String>,
}

impl NewClient {
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_address(&self) -> &Option<String> {
        &self.address
    }

    pub fn get_phone(&self) -> &Option<String> {
        &self.phone
    }

    pub fn get_invoice_email(&self) -> &Option<String> {
        &self.invoice_email
    }
}
