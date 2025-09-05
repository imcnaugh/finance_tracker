use clap::Args;

#[derive(Args, Debug)]
pub struct InitCommand {
    company_name: String,
    company_address: String,
    company_email: String,
}
