use clap::Args;

#[derive(Args)]
pub struct NewClient {
    name: String,
}

impl NewClient {
    pub(crate) fn new(name: String) -> Self {
        Self { name }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}
