use clap::Args;

#[derive(Args)]
pub struct NewAccount {
    name: String,
    account_type_id: i64,
}

impl NewAccount {
    pub fn new(name: String, account_type_id: i64) -> Self {
        Self {
            name,
            account_type_id,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_account_type_id(&self) -> i64 {
        self.account_type_id
    }
}
