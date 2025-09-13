use crate::model::DatabaseConfiguration;
use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};
use std::fs;

// TODO this should really be a result
pub async fn get_pooled_connection(cfg: &DatabaseConfiguration) -> Result<Pool<Sqlite>, String> {
    let db_url = cfg.get_path();
    let max_connections = cfg.get_pool_size();

    if let Some(parent) = std::path::Path::new(&db_url).parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?
    }
    if !std::path::Path::new(&db_url).exists() {
        fs::File::create(&db_url).map_err(|e| e.to_string())?;
    }

    let string = format!("sqlite://{}", &db_url);
    let connection_pool = SqlitePoolOptions::new()
        .max_connections(max_connections)
        .connect(&string)
        .await
        .expect("Failed to connect to database");

    sqlx::query("PRAGMA foreign_keys = ON;")
        .execute(&connection_pool)
        .await
        .expect("Failed to enable foreign keys");

    sqlx::migrate!("./db/migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to run migrations");

    Ok(connection_pool)
}
