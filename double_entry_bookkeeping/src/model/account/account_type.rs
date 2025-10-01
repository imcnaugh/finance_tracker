#[derive(Debug, Clone)]
pub struct AccountType {
    id: i64,
    name: String,
    created_timestamp: i64,
}

impl AccountType {
    pub fn new(id: i64, name: String, created_timestamp: i64) -> Self {
        Self {
            id,
            name,
            created_timestamp,
        }
    }

    pub fn get_id(&self) -> i64 {
        self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_created_timestamp(&self) -> i64 {
        self.created_timestamp
    }
}
