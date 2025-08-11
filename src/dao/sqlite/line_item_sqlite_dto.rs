use crate::dao::crud::Crud;
use crate::dao::line_item_dao::LineItemDao;
use crate::dao::sqlite::sqlite_connection::get_connection;
use crate::model::line_item::LineItem;
use std::error::Error;

pub struct LineItemSqliteDto;

impl LineItemSqliteDto {
    pub fn new() -> Self {
        Self {}
    }
}

impl Crud<LineItem> for LineItemSqliteDto {
    fn create(&self, item: &LineItem) -> Result<(), Box<dyn Error>> {
        let conn = get_connection();

        let mut stmt = conn
            .prepare(
                "INSERT INTO line_items (id, name, price, quantity)
          VALUES (:id, :name, :price, :quantity)",
            )
            .expect("Failed to create statement");

        stmt.bind((":id", item.get_id()))
            .expect("Failed to bind id");
        stmt.bind((":name", item.get_name()))
            .expect("Failed to bind name");
        stmt.bind((":price", item.get_unit_price_in_cents() as i64))
            .expect("Failed to bind price");
        stmt.bind((":quantity", item.get_quantity() as f64))
            .expect("Failed to bind quantity");

        // Execute the statement (for INSERT, you still step once)
        match stmt.next() {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }

    fn read(&self, id: &str) -> Result<LineItem, Box<dyn Error>> {
        todo!()
    }

    fn update(&self, id: &str, item: &LineItem) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn delete(&self, id: &str) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

impl LineItemDao for LineItemSqliteDto {}
