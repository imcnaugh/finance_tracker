use std::error::Error;

pub trait Crud<T> {
    fn create(&self, item: &T) -> Result<(), Box<dyn Error>>;
    fn read(&self, id: &str) -> Result<T, Box<dyn Error>>;
    fn update(&self, id: &str, item: &T) -> Result<(), Box<dyn Error>>;
    fn delete(&self, id: &str) -> Result<(), Box<dyn Error>>;
}
