#[derive(Debug, Clone)]
pub struct NewTransaction {
    debit_account_id: i64,
    credit_account_id: i64,
    amount_in_cents: i64,
    description: String,
}

impl NewTransaction {
    pub fn new(
        debit_account_id: i64,
        credit_account_id: i64,
        amount_in_cents: i64,
        description: String,
    ) -> Self {
        Self {
            debit_account_id,
            credit_account_id,
            amount_in_cents,
            description,
        }
    }

    pub fn get_debit_account_id(&self) -> i64 {
        self.debit_account_id
    }

    pub fn get_credit_account_id(&self) -> i64 {
        self.credit_account_id
    }

    pub fn get_amount_in_cents(&self) -> i64 {
        self.amount_in_cents
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }
}
