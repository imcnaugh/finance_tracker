use crate::dao::sqlite_connection::get_connection;
use crate::model::line_item::LineItem;

pub struct LineItemDao {}

impl LineItemDao {
    pub fn add_line_item(line_item: &LineItem) {
        let conn = get_connection();

        let mut stmt = conn
            .prepare(
                "INSERT INTO line_items (id, name, price, quantity)
         VALUES (:id, :name, :price, :quantity)",
            )
            .expect("Failed to create statement");

        stmt.bind((":id", line_item.get_id()))
            .expect("Failed to bind id");
        stmt.bind((":name", line_item.get_name()))
            .expect("Failed to bind name");
        stmt.bind((":price", line_item.get_unit_price_in_cents() as i64))
            .expect("Failed to bind price");
        stmt.bind((":quantity", line_item.get_quantity() as f64))
            .expect("Failed to bind quantity");

        // Execute the statement (for INSERT, you still step once)
        let _ = stmt.next().expect("Failed to execute statement");
    }
}
