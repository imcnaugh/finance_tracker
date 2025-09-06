use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Configs {
    database: DatabaseConfig,
    company: CompanyConfig,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DatabaseConfig {
    path: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CompanyConfig {
    name: String,
    address: String,
    email: String,
}

impl Configs {
    pub fn new(database_config: DatabaseConfig, company_config: CompanyConfig) -> Self {
        Self {
            database: database_config,
            company: company_config,
        }
    }
}

impl DatabaseConfig {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }
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
