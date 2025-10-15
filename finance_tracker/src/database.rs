use crate::migrations;
use sqlx::{Pool, Sqlite, migrate::MigrateError, sqlite::SqlitePoolOptions};
use std::fs;
use utilities::database_configuration::DatabaseConfiguration;

pub struct DatabaseManager {
    pool: Pool<Sqlite>,
}

impl DatabaseManager {
    pub async fn new(config: &DatabaseConfiguration) -> Result<Self, String> {
        let pool = Self::create_connection_pool(config).await?;

        // Run migrations
        Self::run_migrations(&pool).await?;

        Ok(Self { pool })
    }

    async fn create_connection_pool(
        config: &DatabaseConfiguration,
    ) -> Result<Pool<Sqlite>, String> {
        let db_url = config.get_path();
        let max_connections = config.get_pool_size();

        // Ensure parent directory exists
        if let Some(parent) = std::path::Path::new(&db_url).parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }

        // Create database file if it doesn't exist
        if !std::path::Path::new(&db_url).exists() {
            fs::File::create(&db_url).map_err(|e| e.to_string())?;
        }

        let connection_string = format!("sqlite://{}", &db_url);
        let connection_pool = SqlitePoolOptions::new()
            .max_connections(max_connections)
            .connect(&connection_string)
            .await
            .map_err(|e| e.to_string())?;

        // Enable foreign keys
        sqlx::query("PRAGMA foreign_keys = ON;")
            .execute(&connection_pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(connection_pool)
    }

    async fn run_migrations(pool: &Pool<Sqlite>) -> Result<(), String> {
        sqlx::migrate!("db/migrations")
            .run(pool)
            .await
            .map_err(|e| format!("Failed to run migrations: {e}"))?;

        Ok(())
    }

    pub fn get_pool(&self) -> &Pool<Sqlite> {
        &self.pool
    }
}
