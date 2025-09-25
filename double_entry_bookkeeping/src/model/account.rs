use clap::ValueEnum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Account {
    Cash,
    AccountsReceivable,
    AccountsPayable,
    OwnerEquity,
    OwnerDrawings,
    OperatingExpense,
    Revenues,
    TaxPayable,
    TaxesPaid,
}

pub enum AccountType {
    Asset,
    Liability,
    Equity,
    Revenue,
    Expense,
}

impl Account {
    pub fn account_type(&self) -> AccountType {
        match self {
            Account::Cash | Account::AccountsReceivable | Account::TaxesPaid => AccountType::Asset,

            Account::AccountsPayable | Account::TaxPayable => AccountType::Liability,

            Account::OwnerEquity => AccountType::Equity,
            Account::OwnerDrawings => AccountType::Equity,

            Account::Revenues => AccountType::Revenue,
            Account::OperatingExpense => AccountType::Expense,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpenseType {
    Software,
    Hardware,
    Saas,
    Travel,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaxType {
    FederalIncomeTax,
    StateIncomeTax,
}
