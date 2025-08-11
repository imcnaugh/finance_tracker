pub fn create_sqlite_tables_if_not_exists(conn: &sqlite::Connection) {
    let create_line_item_table_sql = r#"CREATE TABLE IF NOT EXISTS line_items(
        id TEXT PRIMARY KEY,
        name TEXT,
        quantity REAL,
        price INTEGER
    )"#;

    let create_client_table_sql = r#"CREATE TABLE IF NOT EXISTS clients(
        id TEXT PRIMARY KEY,
        name TEXT
    )"#;

    let create_invoice_table_sql = r#"CREATE TABLE IF NOT EXISTS invoices(
        id TEXT PRIMARY KEY,
        client_id TEXT,
        status TEXT,
        FOREIGN KEY(client_id) REFERENCES clients(id)
    )"#;

    let create_invoice_line_item_table_sql = r#"CREATE TABLE IF NOT EXISTS invoice_line_items(
        invoice_id TEXT,
        line_item_id TEXT,
        FOREIGN KEY(invoice_id) REFERENCES invoices(id),
        FOREIGN KEY(line_item_id) REFERENCES line_items(id)
    )"#;

    let _ = conn.execute(create_line_item_table_sql);
    let _ = conn.execute(create_client_table_sql);
    let _ = conn.execute(create_invoice_table_sql);
    let _ = conn.execute(create_invoice_line_item_table_sql);
}
