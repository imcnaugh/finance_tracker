#[derive(Debug, Clone)]
pub struct Account {
    id: u64,
    name: String,
    account_type_id: u64,
    created_timestamp: u64,
}

impl Account {
    pub fn new(id: u64, name: String, account_type_id: u64, created_timestamp: u64) -> Self {
        Self {
            id,
            name,
            account_type_id,
            created_timestamp,
        }
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_account_type_id(&self) -> u64 {
        self.account_type_id
    }

    pub fn get_created_timestamp(&self) -> u64 {
        self.created_timestamp
    }
}
