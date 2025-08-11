use crate::dao::crud::Crud;
use crate::dao::sqlite::line_item_sqlite_dto::LineItemSqliteDto;
use crate::model::NewLineItem;
use crate::model::line_item::LineItem;
use std::error::Error;

pub struct LineItemService {}

impl LineItemService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn log_new_line_item(new_line_item: NewLineItem) -> Result<(), Box<dyn Error>> {
        let dao = LineItemSqliteDto::new();
        let line_item = LineItem::from(new_line_item);
        dao.create(&line_item).expect("TODO: panic message");
        Ok(())
    }
}
