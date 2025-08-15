use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Pool, Sqlite};
use std::env;
use std::sync::OnceLock;

static POOL: OnceLock<Pool<Sqlite>> = OnceLock::new();

fn get_pooled_connection() -> &'static Pool<Sqlite> {
    POOL.get_or_init(|| {
        let db_url = env::var("INVOICE_DB_PATH").expect("INVOICE_DB_PATH not set");
        let max_connections = env::var("INVOICE_DB_POOL_SIZE")
            .unwrap_or(String::from("3"))
            .parse::<u32>()
            .unwrap();

        SqlitePoolOptions::new()
            .max_connections(max_connections)
            .connect_lazy(&db_url)
            .expect("Failed to connect to database")
    })
}
