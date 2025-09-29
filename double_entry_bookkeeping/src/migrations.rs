use sqlx::{Pool, Sqlite, migrate::MigrateError};

pub async fn run(pool: &Pool<Sqlite>) -> Result<(), MigrateError> {
    sqlx::migrate!("../double_entry_bookkeeping/db/migrations")
        .run(pool)
        .await
}
