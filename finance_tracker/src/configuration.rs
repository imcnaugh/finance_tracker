use invoice_manager::model::CompanyConfiguration;
use serde::{Deserialize, Serialize};
use utilities::database_configuration::DatabaseConfiguration;

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    invoice_database_configuration: DatabaseConfiguration,
    bookkeeping_database_configuration: DatabaseConfiguration,
    company_configuration: CompanyConfiguration,
}

impl Configuration {
    pub fn new(
        database_configuration: DatabaseConfiguration,
        bookkeeping_database_configuration: DatabaseConfiguration,
        company_configuration: CompanyConfiguration,
    ) -> Self {
        Self {
            invoice_database_configuration: database_configuration,
            bookkeeping_database_configuration: bookkeeping_database_configuration,
            company_configuration,
        }
    }

    pub fn get_invoice_database_configuration(&self) -> &DatabaseConfiguration {
        &self.invoice_database_configuration
    }

    pub fn get_bookkeeping_database_configuration(&self) -> &DatabaseConfiguration {
        &self.bookkeeping_database_configuration
    }

    pub fn get_company_configuration(&self) -> &CompanyConfiguration {
        &self.company_configuration
    }

    pub fn set_database_configuration(&mut self, database: DatabaseConfiguration) {
        self.invoice_database_configuration = database;
    }

    pub fn set_bookkeeping_database_configuration(&mut self, database: DatabaseConfiguration) {
        self.bookkeeping_database_configuration = database;
    }

    pub fn set_company_configuration(&mut self, company: CompanyConfiguration) {
        self.company_configuration = company;
    }
}
