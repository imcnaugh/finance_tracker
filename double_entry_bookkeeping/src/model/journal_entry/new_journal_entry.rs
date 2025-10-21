use clap::Args;

#[derive(Debug, Clone, Args)]
pub struct NewJournalEntry {
    debit_account_id: u64,
    credit_account_id: u64,
    amount: f64,
    description: String,
}

impl NewJournalEntry {
    pub fn new(
        debit_account_id: u64,
        credit_account_id: u64,
        amount_in_cents: f64,
        description: String,
    ) -> Self {
        Self {
            debit_account_id,
            credit_account_id,
            amount: amount_in_cents,
            description,
        }
    }

    pub fn get_debit_account_id(&self) -> u64 {
        self.debit_account_id
    }

    pub fn get_credit_account_id(&self) -> u64 {
        self.credit_account_id
    }

    pub fn get_amount_in_cents(&self) -> i64 {
        (self.amount * 100.0).round() as i64
    }

    pub fn get_amount(&self) -> f64 {
        self.amount
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }
}
