use sqlx::{Pool, Sqlite, migrate::MigrateError};

pub async fn run(pool: &Pool<Sqlite>) -> Result<(), MigrateError> {
    sqlx::migrate!("../invoice_manager/db/migrations")
        .run(pool)
        .await
}
