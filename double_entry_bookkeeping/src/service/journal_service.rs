use crate::dao::journal_dao::JournalDao;
use crate::model::NewJournalEntry;

pub struct JournalService<J: JournalDao> {
    journal_dao: J,
}

impl<J: JournalDao> JournalService<J> {
    pub fn new(journal_dao: J) -> Self {
        Self { journal_dao }
    }

    pub async fn make_transaction(&self, new_transaction: NewJournalEntry) {
        let journal_entry_result = self.journal_dao.create_journal_entry(new_transaction).await;
        match journal_entry_result {
            Ok(id) => println!("Transaction created with id: {id}"),
            Err(e) => println!("Error creating transaction: {}", e),
        }
        // TODO: Implement transaction creation
        // This would:
        // 1. Create a journal entry
        // 2. Create debit and credit ledger entries
        // 3. Save to database
    }
}
