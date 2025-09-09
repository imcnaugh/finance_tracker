mod company_configuration;
mod database_configuration;

pub use company_configuration::CompanyConfiguration;
pub use database_configuration::DatabaseConfiguration;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    database_configuration: Option<DatabaseConfiguration>,
    company_configuration: Option<CompanyConfiguration>,
}

impl Configuration {
    pub fn new(
        database_configuration: Option<DatabaseConfiguration>,
        company_configuration: Option<CompanyConfiguration>,
    ) -> Self {
        Self {
            database_configuration,
            company_configuration,
        }
    }

    pub fn get_database_configuration(&self) -> Option<&DatabaseConfiguration> {
        self.database_configuration.as_ref()
    }

    pub fn get_company_configuration(&self) -> Option<&CompanyConfiguration> {
        self.company_configuration.as_ref()
    }

    pub fn set_database_configuration(&mut self, database: DatabaseConfiguration) {
        self.database_configuration = Some(database);
    }

    pub fn set_company_configuration(&mut self, company: CompanyConfiguration) {
        self.company_configuration = Some(company);
    }
}
