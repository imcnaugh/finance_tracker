use crate::dao::sqlite::create_sqlite_tables::create_sqlite_tables_if_not_exists;
use std::env;
use std::sync::{Mutex, MutexGuard, OnceLock};

static DB: OnceLock<Mutex<sqlite::Connection>> = OnceLock::new();

fn db() -> &'static Mutex<sqlite::Connection> {
    DB.get_or_init(|| {
        let db_path = env::var("INVOICE_DB_PATH").expect("INVOICE_DB_PATH not set");
        let conn = sqlite::Connection::open(&db_path).expect("failed to open DB");
        // Optional pragmas
        let _ = conn.execute("PRAGMA journal_mode = WAL;");
        let _ = conn.execute("PRAGMA synchronous = NORMAL;");
        let _ = conn.execute("PRAGMA busy_timeout = 5000;");
        let _ = conn.execute("PRAGMA foreign_keys = ON;");
        create_sqlite_tables_if_not_exists(&conn);
        Mutex::new(conn)
    })
}

// Callers can get a locked connection guard:
pub fn get_connection() -> MutexGuard<'static, sqlite::Connection> {
    db().lock().expect("DB mutex poisoned")
}
