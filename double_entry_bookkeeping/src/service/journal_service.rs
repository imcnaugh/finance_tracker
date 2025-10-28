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

    #[test]
    fn test_new_imbalance_transaction() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let credit_amount = 100;
            let debit_amount = 99;

            let credit_transaction = NewTransaction::new(0, credit_amount, false);
            let debit_transaction = NewTransaction::new(0, debit_amount, true);

            let imbalanced_transaction = NewJournalEntry::new(
                "test transaction".to_string(),
                vec![credit_transaction, debit_transaction],
            );

            let mock_dao = MockDao::new();
            let mock_dao = Arc::new(mock_dao);
            let journal_service = JournalService::new(mock_dao.clone());

            let response = journal_service
                .make_transaction(imbalanced_transaction)
                .await;

            assert!(response.is_err());
        });
    }

    struct MockDao {}

    impl MockDao {
        fn new() -> Self {
            MockDao {}
        }
    }

    impl JournalDao for MockDao {
        async fn create_journal_entry(
            &self,
            new_journal_entry: NewJournalEntry,
        ) -> Result<u64, Error> {
            todo!()
        }

        async fn get_journal_entry_by_id(
            &self,
            journal_entry_id: u64,
        ) -> Result<Option<JournalEntry>, Error> {
            todo!()
        }

        async fn get_transactions_by_journal_entry_id(
            &self,
            journal_entry_id: u64,
        ) -> Result<Vec<Transaction>, Error> {
            todo!()
        }

        async fn get_account_balance(&self, account_id: u64) -> Result<i64, Error> {
            todo!()
        }
    }
}
