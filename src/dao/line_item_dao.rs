use crate::dao::crud::Crud;
use crate::model::line_item::LineItem;

pub trait LineItemDao: Crud<LineItem> {}
