use crate::dao::journal_dao::JournalDao;
use crate::model::NewJournalEntry;
use std::sync::Arc;

pub struct JournalService<J: JournalDao> {
    journal_dao: Arc<J>,
}

impl<J> JournalService<J>
where
    J: JournalDao,
{
    pub fn new(journal_dao: Arc<J>) -> Self {
        Self { journal_dao }
    }

    /// Asynchronously processes a new journal transaction by validating its debit and credit transactions
    /// and storing it in the database if valid.
    ///
    /// # Arguments
    ///
    /// * `new_transaction` - A `NewJournalEntry` instance containing the transaction data.
    ///
    /// # Returns
    ///
    /// Returns a `Result`:
    /// - `Ok(u64)` containing the unique ID of the created journal entry if the operation succeeds.
    /// - `Err(String)` containing an error message if the transaction fails validation or a database error occurs.
    ///
    /// # Validation
    ///
    /// The function checks whether the total debit amount matches the total credit amount:
    /// - If the sums do not match, an error is returned with the message "Debit and credit sums do not match".
    ///
    /// # Errors
    ///
    /// This function can return an error in the following scenarios:
    /// - If the debit and credit amounts are not equal.
    /// - If there is an issue during the database operation to create the journal entry.
    pub async fn make_transaction(&self, new_transaction: NewJournalEntry) -> Result<u64, String> {
        let (debit_sum, credit_sum) =
            &new_transaction
                .get_transactions()
                .iter()
                .fold((0, 0), |total, transaction| {
                    if transaction.is_debit() {
                        (total.0 + transaction.get_amount_in_cents(), total.1)
                    } else {
                        (total.0, total.1 + transaction.get_amount_in_cents())
                    }
                });

        if debit_sum != credit_sum {
            return Err("Debit and credit sums do not match".to_string());
        }

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{JournalEntry, NewTransaction, Transaction};
    use sqlx::Error;
    use std::cell::RefCell;

    #[test]
    fn test_new_imbalance_transaction() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let (mock_dao, journal_service) = test_setup();

            let credit_amount = 100;
            let debit_amount = 99;

            let credit_transaction = NewTransaction::new(0, credit_amount, false);
            let debit_transaction = NewTransaction::new(0, debit_amount, true);

            let imbalanced_transaction = NewJournalEntry::new(
                "test transaction".to_string(),
                vec![credit_transaction, debit_transaction],
            );

            let response = journal_service
                .make_transaction(imbalanced_transaction)
                .await;

            assert!(response.is_err());
            assert_eq!(response.unwrap_err(), "Debit and credit sums do not match");
            assert!(mock_dao.created_journal_entries.borrow().is_empty());
        });
    }

    #[test]
    fn test_new_transaction_many_credits_and_debits() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let (mock_dao, journal_service) = test_setup();

            let credit_transaction_1 = NewTransaction::new(0, 832, false);
            let credit_transaction_2 = NewTransaction::new(0, 2, false);
            let credit_transaction_3 = NewTransaction::new(0, 50, false);
            let debit_transaction_1 = NewTransaction::new(0, 734, true);
            let debit_transaction_2 = NewTransaction::new(0, 150, true);

            let transaction = NewJournalEntry::new(
                "test transaction".to_string(),
                vec![
                    credit_transaction_1,
                    credit_transaction_2,
                    credit_transaction_3,
                    debit_transaction_1,
                    debit_transaction_2,
                ],
            );

            let response = journal_service.make_transaction(transaction.clone()).await;

            assert!(response.is_ok());
            assert_eq!(response.unwrap(), 1);
            assert_eq!(mock_dao.created_journal_entries.borrow().len(), 1);
            assert_eq!(mock_dao.created_journal_entries.borrow()[0], transaction);
        });
    }

    fn test_setup() -> (Arc<MockDao>, JournalService<MockDao>) {
        let mock_dao = MockDao::new();
        let mock_dao = Arc::new(mock_dao);
        let journal_service = JournalService::new(mock_dao.clone());
        (mock_dao, journal_service)
    }

    struct MockDao {
        created_journal_entries: RefCell<Vec<NewJournalEntry>>,
    }

    impl MockDao {
        fn new() -> Self {
            MockDao {
                created_journal_entries: RefCell::new(vec![]),
            }
        }
    }

    impl JournalDao for MockDao {
        async fn create_journal_entry(
            &self,
            new_journal_entry: NewJournalEntry,
        ) -> Result<u64, Error> {
            self.created_journal_entries
                .borrow_mut()
                .push(new_journal_entry);
            Ok(1)
        }

        async fn get_journal_entry_by_id(
            &self,
            _journal_entry_id: u64,
        ) -> Result<Option<JournalEntry>, Error> {
            Ok(None)
        }

        async fn get_transactions_by_journal_entry_id(
            &self,
            _journal_entry_id: u64,
        ) -> Result<Vec<Transaction>, Error> {
            Ok(vec![])
        }

        async fn get_account_balance(&self, _account_id: u64) -> Result<i64, Error> {
            Ok(0)
        }
    }
}
