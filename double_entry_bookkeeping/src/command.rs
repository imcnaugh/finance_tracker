use clap::{Args, Subcommand};

#[derive(Args, Debug)]
pub struct AccountingArgs {
    #[command(subcommand)]
    pub command: AccountingCommands,
}

#[derive(Subcommand, Debug)]
pub enum AccountingCommands {
    /// Create a new transaction
    Transaction(TransactionArgs),
    /// List accounts
    Accounts,
    /// List account types
    AccountTypes,
}

#[derive(Args, Debug)]
pub struct TransactionArgs {
    /// ID of the debit account
    #[arg(long)]
    pub debit_account_id: i64,

    /// ID of the credit account
    #[arg(long)]
    pub credit_account_id: i64,

    /// Amount in cents
    #[arg(long)]
    pub amount_cents: i64,

    /// Description of the transaction
    #[arg(long)]
    pub description: String,
}