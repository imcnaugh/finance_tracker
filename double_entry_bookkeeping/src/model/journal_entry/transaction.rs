use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Transaction {
    id: u64,
    account_id: u64,
    journal_entry_id: u64,
    amount_in_cents: i64,
    is_debit: bool,
    created_timestamp: i64,
}

impl Transaction {
    pub fn new(
        id: u64,
        account_id: u64,
        journal_id: u64,
        amount_in_cents: i64,
        is_debit: bool,
        created_timestamp: i64,
    ) -> Self {
        Self {
            id,
            account_id,
            journal_entry_id: journal_id,
            amount_in_cents,
            is_debit,
            created_timestamp,
        }
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn get_account_id(&self) -> u64 {
        self.account_id
    }

    pub fn get_journal_entry_id(&self) -> u64 {
        self.journal_entry_id
    }

    pub fn get_amount_in_cents(&self) -> i64 {
        self.amount_in_cents
    }

    pub fn is_debit(&self) -> bool {
        self.is_debit
    }

    pub fn get_created_timestamp(&self) -> i64 {
        self.created_timestamp
    }
}
