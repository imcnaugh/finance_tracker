use crate::model::journal_entry::new_transaction::NewTransaction;

#[derive(Debug, Clone)]
pub struct NewJournalEntry {
    description: String,
    transactions: Vec<NewTransaction>,
}

impl NewJournalEntry {
    pub fn new(description: String, transactions: Vec<NewTransaction>) -> Self {
        Self {
            description,
            transactions,
        }
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_transactions(&self) -> &Vec<NewTransaction> {
        &self.transactions
    }
}
