use crate::model::account::account_structural_type::AccountStructuralType;
use crate::model::{AccountType, Transaction};
use sqlx::{FromRow, Row};

#[derive(Debug, Clone)]
pub struct Account {
    id: u64,
    name: String,
    account_type: AccountType,
    transitions: Vec<Transaction>,
    structural_class: AccountStructuralType,
    balance_in_cents: i64,
    created_timestamp: u64,
}

impl Account {
    pub fn new(
        id: u64,
        name: String,
        account_type: AccountType,
        transitions: Vec<Transaction>,
        structural_class: AccountStructuralType,
        balance_in_cents: i64,
        created_timestamp: u64,
    ) -> Self {
        Self {
            id,
            name,
            account_type,
            transitions,
            structural_class,
            balance_in_cents,
            created_timestamp,
        }
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_account_type(&self) -> &AccountType {
        &self.account_type
    }

    pub fn get_transitions(&self) -> &Vec<Transaction> {
        &self.transitions
    }

    pub fn set_transitions(&mut self, transitions: Vec<Transaction>) {
        self.transitions = transitions;
    }

    pub fn get_balance_in_cents(&self) -> i64 {
        self.balance_in_cents
    }

    pub fn get_created_timestamp(&self) -> u64 {
        self.created_timestamp
    }
}

impl<'r> FromRow<'r, sqlx::sqlite::SqliteRow> for Account {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> sqlx::Result<Self> {
        Ok(Self {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            transitions: vec![],
            balance_in_cents: row.try_get("balance_in_cents")?,
            created_timestamp: row.try_get("created_timestamp")?,
            account_type: AccountType::new(
                row.try_get("at_id")?,
                row.try_get("at_name")?,
                row.try_get("at_normal_balance")?,
                row.try_get("at_created_timestamp")?,
            ),
            structural_class: row.try_get("structural_class")?,
        })
    }
}
