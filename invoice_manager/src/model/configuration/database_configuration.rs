use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct DatabaseConfiguration {
    path: String,
    pool_size: Option<u32>,
}

impl DatabaseConfiguration {
    pub fn new(path: &str, pool_size: Option<u32>) -> Self {
        Self {
            path: path.to_string(),
            pool_size,
        }
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn get_pool_size(&self) -> u32 {
        self.pool_size.unwrap_or(3)
    }
}
