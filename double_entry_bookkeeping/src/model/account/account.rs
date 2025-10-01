#[derive(Debug, Clone)]
pub struct Account {
    id: i64,
    name: String,
    account_type_id: i64,
    created_timestamp: i64,
}

impl Account {
    pub fn new(id: i64, name: String, account_type_id: i64, created_timestamp: i64) -> Self {
        Self {
            id,
            name,
            account_type_id,
            created_timestamp,
        }
    }

    pub fn get_id(&self) -> i64 {
        self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_account_type_id(&self) -> i64 {
        self.account_type_id
    }

    pub fn get_created_timestamp(&self) -> i64 {
        self.created_timestamp
    }
}
