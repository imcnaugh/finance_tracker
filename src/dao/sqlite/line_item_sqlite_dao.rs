use crate::dao::crud::Crud;
use crate::dao::line_item_dao::LineItemDao;
use crate::dao::sqlite::sqlite_connection::get_connection;
use crate::model::line_item::LineItem;
use sqlite::ColumnIndex;
use std::error::Error;

pub struct LineItemSqliteDao;

impl LineItemSqliteDao {
    pub fn new() -> Self {
        Self {}
    }
}

impl Crud<LineItem> for LineItemSqliteDao {
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

        match stmt.next() {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }

    fn read(&self, id: &str) -> Result<LineItem, Box<dyn Error>> {
        let conn = get_connection();

        let mut stmt = conn
            .prepare("select id, name, quantity, price from line_items where id=:id")
            .expect("Failed to prepare statement")
            .into_iter()
            .bind((":id", id))
            .expect("Failed to bind id");

        match stmt.next() {
            None => {
                todo!()
            }
            Some(row) => {
                let row = row.expect("Failed to get row");
                let id = row.read::<&str, _>("id").into();
                let name = row.read::<&str, _>("name").into();
                let quantity = row.read::<f64, _>("quantity");
                let price = row.read::<i64, _>("price");
                let line_item = LineItem::new(id, name, price as isize, quantity as f32);
                Ok(line_item)
            }
        }
    }

    fn update(&self, id: &str, item: &LineItem) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn delete(&self, id: &str) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

impl LineItemDao for LineItemSqliteDao {}
