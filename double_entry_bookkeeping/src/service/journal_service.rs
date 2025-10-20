use crate::dao::journal_dao::JournalDao;
use crate::model::NewJournalEntry;

pub struct JournalService<J: JournalDao> {
    journal_dao: J,
}

impl<J: JournalDao> JournalService<J> {
    pub fn new(journal_dao: J) -> Self {
        Self { journal_dao }
    }

    pub async fn make_transaction(&self, new_transaction: NewJournalEntry) -> Result<u64, String> {
        let journal_entry_id = self
            .journal_dao
            .create_journal_entry(new_transaction)
            .await
            .map_err(|e| e.to_string())?;

        Ok(journal_entry_id)
    }

    pub async fn get_account_balance(&self, account_id: u64) -> Result<i64, String> {
        let balance = self
            .journal_dao
            .get_account_balance(account_id)
            .await
            .map_err(|e| e.to_string())?;

        Ok(balance)
    }
}
