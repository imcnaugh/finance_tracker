use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct DatabaseConfiguration {
    path: String,
}

impl DatabaseConfiguration {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }
}
