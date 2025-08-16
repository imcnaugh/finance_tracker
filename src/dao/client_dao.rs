use crate::dao::crud::Crud;
use crate::model::client::Client;
use sqlx::Database;

pub trait ClientDao<DB: Database>: Crud<Client, DB> {}
