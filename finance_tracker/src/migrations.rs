use sqlx::{Pool, Sqlite, migrate::MigrateError};

pub async fn run(pool: &Pool<Sqlite>) -> Result<(), MigrateError> {
    sqlx::migrate!("./db/migrations").run(pool).await
}
