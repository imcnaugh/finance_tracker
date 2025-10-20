use crate::model::account::NormalBalance;
use clap::Args;

#[derive(Args)]
pub struct NewAccountType {
    name: String,
    normal_balance: NormalBalance,
}

impl NewAccountType {
    pub fn new(name: String, normal_balance: NormalBalance) -> Self {
        Self {
            name,
            normal_balance,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_normal_balance(&self) -> &NormalBalance {
        &self.normal_balance
    }
}
