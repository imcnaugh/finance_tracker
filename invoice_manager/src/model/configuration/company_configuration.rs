use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CompanyConfiguration {
    name: String,
    address: String,
    email: String,
}

impl CompanyConfiguration {
    pub fn new(name: &str, address: &str, email: &str) -> Self {
        Self {
            name: name.to_string(),
            address: address.to_string(),
            email: email.to_string(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_address(&self) -> &str {
        &self.address
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }
}
