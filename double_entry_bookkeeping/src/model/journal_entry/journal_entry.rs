#[derive(Debug, Clone)]
pub struct JournalEntry {
    id: u64,
    description: String,
    created_timestamp: i64,
}

impl JournalEntry {
    pub fn new(id: u64, description: String, created_timestamp: i64) -> Self {
        Self {
            id,
            description,
            created_timestamp,
        }
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_created_timestamp(&self) -> i64 {
        self.created_timestamp
    }
}
