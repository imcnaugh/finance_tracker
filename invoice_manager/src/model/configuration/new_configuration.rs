use clap::Args;

#[derive(Args, Debug)]
pub struct NewCompanyConfiguration {
    company_name: String,
    company_address: String,
    company_email: String,
}

impl NewCompanyConfiguration {
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
