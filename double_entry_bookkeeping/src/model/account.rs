use crate::model::account_type::AccountType;

pub struct Account {
    id: usize,
    name: String,
    account_type: AccountType,
    created_timestamp: i64,
}
