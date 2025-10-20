use crate::model::account::normal_balance::NormalBalance;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct AccountType {
    id: u64,
    name: String,
    normal_balance: NormalBalance,
    created_timestamp: u64,
}

impl AccountType {
    pub fn new(
        id: u64,
        name: String,
        normal_balance: NormalBalance,
        created_timestamp: u64,
    ) -> Self {
        Self {
            id,
            name,
            normal_balance,
            created_timestamp,
        }
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_normal_balance(&self) -> &NormalBalance {
        &self.normal_balance
    }

    pub fn get_created_timestamp(&self) -> u64 {
        self.created_timestamp
    }
}
