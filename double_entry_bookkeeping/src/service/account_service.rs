use crate::dao::account_dao::AccountDao;
use crate::dao::journal_dao::JournalDao;
use crate::model::{Account, AccountStructuralType, AccountType, NewJournalEntry, NewTransaction};
use std::sync::Arc;

pub struct AccountService<A: AccountDao> {
    account_dao: Arc<A>,
}

impl<A: AccountDao> AccountService<A> {
    pub fn new(account_dao: Arc<A>) -> Self {
        Self { account_dao }
    }

    pub async fn get_all_account_types(&self) -> Result<Vec<AccountType>, String> {
        let account_types = self
            .account_dao
            .get_all_account_types()
            .await
            .map_err(|e| e.to_string())?;

        Ok(account_types)
    }

    pub async fn get_all_accounts(&self) -> Result<Vec<Account>, String> {
        let accounts = self
            .account_dao
            .get_all_accounts()
            .await
            .map_err(|e| e.to_string())?;

        Ok(accounts)
    }

    pub async fn get_account_by_id(&self, account_id: u64) -> Result<Account, String> {
        self.account_dao
            .get_account_by_id(account_id)
            .await
            .map_err(|e| e.to_string())
            .and_then(|opt| opt.ok_or_else(|| "Account not found".to_string()))
    }

    pub async fn close_accounts<J: JournalDao>(
        &self,
        journal_dao: &J,
    ) -> Result<(), String> {
        // Get all accounts
        let accounts = self.get_all_accounts().await?;

        // Find the owner equity account to receive the closing balance
        let owner_equity_account = accounts
            .iter()
            .find(|account| {
                account.get_account_type().get_name() == "equity"
                    && account.get_name() == "owner equity"
            })
            .ok_or_else(|| "Owner equity account not found".to_string())?;

        // Collect all temporary accounts (revenue and expense) with non-zero balances
        let temporary_accounts: Vec<&Account> = accounts
            .iter()
            .filter(|account| {
                let type_name = account.get_account_type().get_name();
                (type_name == "revenue" || type_name == "expense")
                    && account.get_balance_in_cents() != 0
            })
            .collect();

        if temporary_accounts.is_empty() {
            return Ok(());
        }

        // Calculate net income (revenue - expense)
        let net_income = temporary_accounts.iter().fold(0i64, |acc, account| {
            // Revenue accounts have credit normal balance and positive balance means credit
            // Expense accounts have debit normal balance and positive balance means debit
            // Net income = Revenue - Expense
            if account.get_account_type().get_name() == "revenue" {
                acc + account.get_balance_in_cents()
            } else {
                // expense
                acc - account.get_balance_in_cents()
            }
        });

        // Create transactions to close each temporary account
        let mut transactions = Vec::new();

        for account in temporary_accounts {
            let balance = account.get_balance_in_cents();
            if balance == 0 {
                continue;
            }

            // To close an account, we need to create a transaction that zeros it out
            // If the account has a positive balance, we need to do the opposite to zero it
            let is_debit = if account.get_account_type().get_name() == "revenue" {
                // Revenue has credit balance, so we debit to close it
                true
            } else {
                // Expense has debit balance, so we credit to close it
                false
            };

            transactions.push(NewTransaction::new(
                account.get_id(),
                balance.abs() as i64,
                is_debit,
            ));
        }

        // Add the balancing transaction to owner equity
        // If net income is positive (profit), credit owner equity
        // If net income is negative (loss), debit owner equity
        transactions.push(NewTransaction::new(
            owner_equity_account.get_id(),
            net_income.abs() as i64,
            net_income < 0, // if negative, debit; if positive, credit
        ));

        // Create the journal entry
        let journal_entry = NewJournalEntry::new(
            "Closing entries - transferring revenue and expense to owner equity".to_string(),
            transactions,
        );

        journal_dao
            .create_journal_entry(journal_entry)
            .await
            .map_err(|e| format!("Failed to create closing journal entry: {}", e))?;

        Ok(())
    }

    async fn validate_accounting_equation(&self) -> bool {
        match self.account_dao.get_all_accounts().await {
            Ok(accounts) => {
                let (assets, liabilities, equity) = accounts.iter().fold(
                    (0i64, 0i64, 0i64),
                    |(asset, liability, equity), account| {
                        match account.get_structural_class() {
                            AccountStructuralType::Asset => {
                                (asset + account.get_balance_in_cents(), liability, equity)
                            }
                            AccountStructuralType::Liability => {
                                (asset, liability + account.get_balance_in_cents(), equity)
                            }
                            AccountStructuralType::Equity => {
                                (asset, liability, equity + account.get_balance_in_cents())
                            }
                        }
                    },
                );
                assets == liabilities + equity
            }
            Err(_) => false,
        }
    }
}
