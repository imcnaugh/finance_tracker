use crate::model::client::NewClient;
use crate::utils::generate_new_id;

#[derive(Debug, sqlx::FromRow)]
pub struct Client {
    id: String,
    name: String,
    address: Option<String>,
    phone: Option<String>,
    invoice_email: Option<String>,
}

impl From<NewClient> for Client {
    fn from(value: NewClient) -> Self {
        Self {
            id: generate_new_id(),
            name: value.get_name().into(),
            address: value.get_address().clone(),
            phone: value.get_phone().clone(),
            invoice_email: value.get_invoice_email().clone(),
        }
    }
}

impl Client {
    pub fn get_id(&self) -> &str {
        self.id.as_str()
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_address(&self) -> Option<&str> {
        self.address.as_deref()
    }

    pub fn get_phone(&self) -> Option<&str> {
        self.phone.as_deref()
    }

    pub fn get_invoice_email(&self) -> Option<&str> {
        self.invoice_email.as_deref()
    }
}
