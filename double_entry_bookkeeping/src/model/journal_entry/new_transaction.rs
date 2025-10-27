#[derive(Debug, Clone)]
pub struct NewTransaction {
    account_id: u64,
    amount_in_cents: i64,
    is_debit: bool,
}

impl NewTransaction {
    pub fn new(account_id: u64, amount_in_cents: i64, is_debit: bool) -> Self {
        Self {
            account_id,
            amount_in_cents,
            is_debit,
        }
    }

    pub fn get_account_id(&self) -> u64 {
        self.account_id
    }
    pub fn get_amount_in_cents(&self) -> i64 {
        self.amount_in_cents
    }
    pub fn is_debit(&self) -> bool {
        self.is_debit
    }
}
