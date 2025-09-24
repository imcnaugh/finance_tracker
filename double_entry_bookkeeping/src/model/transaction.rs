use crate::account::Account;

pub struct Transaction {
    id: usize,
    account: Account,
    timestamp: i64,
    description: String,
    debit_in_cents: i64,
    credit_in_cents: i64,
}
