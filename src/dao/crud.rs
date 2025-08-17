use sqlx::{Database, Executor};
use std::error::Error;

/// A generic trait for CRUD (Create, Read, Update, Delete) operations in a database.
///
/// # Type Parameters
/// * `T` - The type of item being stored/retrieved
/// * `DB` - The database type that implements the `Database` trait
pub trait Crud<T> {
    type DB: Database;

    /// Creates a new item in the database.
    ///
    /// # Arguments
    /// * `executor` - The database executor to use for the query
    /// * `item` - The item to create
    ///
    /// # Returns
    /// A Result containing () if successful, or an error if the operation fails
    async fn create<'e, E>(&self, executor: E, item: &T) -> Result<(), Box<dyn Error>>
    where
        E: Executor<'e, Database = Self::DB>;

    /// Retrieves an item from the database by its ID.
    ///
    /// # Arguments
    /// * `executor` - The database executor to use for the query
    /// * `id` - The unique identifier of the item to retrieve
    ///
    /// # Returns
    /// A Result containing an Option with the item if found, or None if not found
    async fn read<'e, E>(&self, executor: E, id: &str) -> Result<Option<T>, Box<dyn Error>>
    where
        E: Executor<'e, Database = Self::DB>;

    /// Updates an existing item in the database.
    ///
    /// # Arguments
    /// * `executor` - The database executor to use for the query
    /// * `id` - The unique identifier of the item to update
    /// * `item` - The new data to update the item with
    ///
    /// # Returns
    /// A Result containing () if successful, or an error if the operation fails
    async fn update<'e, E>(&self, executor: E, id: &str, item: &T) -> Result<(), Box<dyn Error>>
    where
        E: Executor<'e, Database = Self::DB>;

    /// Deletes an item from the database by its ID.
    ///
    /// # Arguments
    /// * `executor` - The database executor to use for the query
    /// * `id` - The unique identifier of the item to delete
    ///
    /// # Returns
    /// A Result containing () if successful, or an error if the operation fails
    async fn delete<'e, E>(&self, executor: E, id: &str) -> Result<(), Box<dyn Error>>
    where
        E: Executor<'e, Database = Self::DB>;
}
