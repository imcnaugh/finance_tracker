use clap::Args;

#[derive(Args)]
pub struct NewAccountType {
    name: String,
}

impl NewAccountType {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}
