use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub struct DatabaseError {
    pub message: String,
}

impl DatabaseError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Debug for DatabaseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Database Error: {}", self.message)
    }
}

impl Display for DatabaseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Database Error: {}", self.message)
    }
}

impl Error for DatabaseError {}
