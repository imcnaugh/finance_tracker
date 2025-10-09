use invoice_manager::model::CompanyConfiguration;
use serde::{Deserialize, Serialize};
use utilities::database_configuration::DatabaseConfiguration;

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    database_configuration: DatabaseConfiguration,
    company_configuration: CompanyConfiguration,
}

impl Configuration {
    pub fn new(
        database_configuration: DatabaseConfiguration,
        company_configuration: CompanyConfiguration,
    ) -> Self {
        Self {
            database_configuration,
            company_configuration,
        }
    }

    pub fn get_database_configuration(&self) -> &DatabaseConfiguration {
        &self.database_configuration
    }

    pub fn get_company_configuration(&self) -> &CompanyConfiguration {
        &self.company_configuration
    }

    pub fn set_database_configuration(&mut self, database: DatabaseConfiguration) {
        self.database_configuration = database;
    }

    pub fn set_company_configuration(&mut self, company: CompanyConfiguration) {
        self.company_configuration = company;
    }
}
