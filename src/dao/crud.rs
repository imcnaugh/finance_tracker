use sqlx::{Database, Executor};
use std::error::Error;

pub trait Crud<T, DB: Database> {
    async fn create<'e, E>(&self, executor: E, item: &T) -> Result<(), Box<dyn Error>>
    where
        E: Executor<'e, Database = DB>;
    async fn read<'e, E>(&self, executor: E, id: &str) -> Result<T, Box<dyn Error>>
    where
        E: Executor<'e, Database = DB>;
    async fn update<'e, E>(&self, executor: E, id: &str, item: &T) -> Result<(), Box<dyn Error>>
    where
        E: Executor<'e, Database = DB>;
    async fn delete<'e, E>(&self, executor: E, id: &str) -> Result<(), Box<dyn Error>>
    where
        E: Executor<'e, Database = DB>;
}
