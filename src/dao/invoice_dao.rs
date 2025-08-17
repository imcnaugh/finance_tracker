use crate::dao::crud::Crud;
use crate::model::invoice::Invoice;
use sqlx::Database;

pub trait InvoiceDao<DB: Database>: Crud<Invoice, DB> {}
