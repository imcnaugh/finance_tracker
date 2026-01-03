use crate::model::account::account_structural_type::AccountStructuralType;
use clap::Args;

#[derive(Args, Clone)]
pub struct NewAccount {
    name: String,
    account_type_id: u64,
    structural_class: AccountStructuralType,
}

impl NewAccount {
    pub fn new(
        name: String,
        account_type_id: u64,
        structural_class: AccountStructuralType,
    ) -> Self {
        Self {
            name,
            account_type_id,
            structural_class,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_account_type_id(&self) -> u64 {
        self.account_type_id
    }

    pub fn get_structural_class(&self) -> &AccountStructuralType {
        &self.structural_class
    }
}
