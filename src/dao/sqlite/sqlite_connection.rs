use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};
use std::env;
use std::fs;
use tokio::sync::OnceCell;

static POOL: OnceCell<Pool<Sqlite>> = OnceCell::const_new();

pub async fn get_pooled_connection() -> &'static Pool<Sqlite> {
    POOL.get_or_init(|| async {
        let db_url = env::var("INVOICE_DB_PATH").expect("INVOICE_DB_PATH not set");
        let max_connections = env::var("INVOICE_DB_POOL_SIZE")
            .unwrap_or(String::from("3"))
            .parse::<u32>()
            .unwrap();

        if let Some(parent) = std::path::Path::new(&db_url).parent() {
            fs::create_dir_all(parent).expect("Failed to create database directory");
        }
        if !std::path::Path::new(&db_url).exists() {
            fs::File::create(&db_url).expect("Failed to create database file");
        }


        let string = format!("sqlite://{}", &db_url);
        let connection_pool = SqlitePoolOptions::new()
            .max_connections(max_connections)
            .connect(&string)
            .await
            .expect("Failed to connect to database");

        sqlx::migrate!("./db/migrations")
            .run(&connection_pool)
            .await
            .expect("Failed to run migrations");

        connection_pool
    })
    .await
}
