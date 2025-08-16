use std::error::Error;

pub trait Crud<T> {
    async fn create(&self, item: &T) -> Result<(), Box<dyn Error>>;
    async fn read(&self, id: &str) -> Result<T, Box<dyn Error>>;
    async fn update(&self, id: &str, item: &T) -> Result<(), Box<dyn Error>>;
    async fn delete(&self, id: &str) -> Result<(), Box<dyn Error>>;
}
