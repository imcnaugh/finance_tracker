use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CompanyConfig {
    name: String,
    address: String,
    email: String,
}

impl CompanyConfig {
    pub fn new(name: &str, address: &str, email: &str) -> Self {
        Self {
            name: name.to_string(),
            address: address.to_string(),
            email: email.to_string(),
        }
    }
}
