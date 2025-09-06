use clap::Args;

#[derive(Args, Debug)]
pub struct NewConfig {
    company_name: String,
    company_address: String,
    company_email: String,
}

impl NewConfig {
    pub fn get_company_name(&self) -> &str {
        &self.company_name
    }
    pub fn get_company_address(&self) -> &str {
        &self.company_address
    }
    pub fn get_company_email(&self) -> &str {
        &self.company_email
    }
}
