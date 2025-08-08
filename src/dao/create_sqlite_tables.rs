pub fn create_sqlite_tables_if_not_exists(conn: &sqlite::Connection) {
    let _ = conn.execute("CREATE TABLE IF NOT EXISTS line_items (id TEXT PRIMARY KEY, name TEXT, quantity REAL, price INTEGER)");
}
