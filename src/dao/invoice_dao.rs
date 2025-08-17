use crate::dao::crud::Crud;
use crate::model::invoice::Invoice;

pub trait InvoiceDao: Crud<Invoice> {}
