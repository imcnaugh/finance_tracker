use crate::dao::crud::Crud;
use crate::model::client::Client;

pub trait ClientDao: Crud<Client> {}
