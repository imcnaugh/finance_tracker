use chrono::{DateTime, Utc};
use comfy_table::Table;
use comfy_table::presets::UTF8_FULL;
use double_entry_bookkeeping::model::Account;
use utilities::Error;

pub struct AccountDisplay {
    id: String,
    name: String,
    balance: String,
    account_type: String,
    normal_balance: String,
}

pub struct TransactionDisplay {
    id: String,
    date: String,
    description: String,
    debit: String,
    credit: String,
}

impl From<&Account> for AccountDisplay {
    fn from(value: &Account) -> Self {
        let total_in_cents = format!("${:.2}", value.get_balance_in_cents() as f64 / 100.0);
        Self {
            id: value.get_id().to_string(),
            name: value.get_name().to_string(),
            balance: total_in_cents,
            account_type: value.get_account_type().get_name().to_string(),
            normal_balance: value.get_account_type().get_normal_balance().to_string(),
        }
    }
}

impl From<&double_entry_bookkeeping::model::Transaction> for TransactionDisplay {
    fn from(value: &double_entry_bookkeeping::model::Transaction) -> Self {
        let display_amount = format!("${:.2}", value.get_amount_in_cents() as f64 / 100.0);
        let (credit, debit) = match value.is_debit() {
            true => (String::new(), display_amount),
            false => (display_amount, String::new()),
        };
        Self {
            id: value.get_id().to_string(),
            date: date_result_to_string(value.get_created_timestamp()),
            description: value.get_description().to_string(),
            debit,
            credit,
        }
    }
}

pub fn display_account(account: &Account) {
    let ad = AccountDisplay::from(account);
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);

    table.add_row(vec!["ID", ad.id.as_str()]);
    table.add_row(vec!["Name", ad.name.as_str()]);
    table.add_row(vec!["Balance", ad.balance.as_str()]);
    table.add_row(vec!["Account Type", ad.account_type.as_str()]);
    table.add_row(vec!["Normal Balance", ad.normal_balance.as_str()]);

    println!("Account:");
    println!("{}", table);

    display_transactions(account.get_transitions());
}

pub fn display_accounts(accounts: &[Account]) {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec!["ID", "Name", "Balance", "Account Type"]);

    for acc in accounts {
        let ad = AccountDisplay::from(acc);
        table.add_row(vec![ad.id, ad.name, ad.balance, ad.account_type]);
    }

    println!("Accounts:");
    if table.is_empty() {
        println!("No accounts found");
    } else {
        println!("{}", table);
    }
}

pub fn display_transactions(transactions: &[double_entry_bookkeeping::model::Transaction]) {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec!["ID", "Date", "Description", "Debit", "Credit"]);

    for transaction in transactions {
        let td = TransactionDisplay::from(transaction);
        table.add_row(vec![td.id, td.date, td.description, td.debit, td.credit]);
    }

    println!("Transactions:");
    if table.is_empty() {
        println!("No transactions found");
    } else {
        println!("{}", table);
    }
}

fn date_result_to_string(date: Result<DateTime<Utc>, Error>) -> String {
    date.ok()
        .map(|d| d.format("%Y-%m-%d").to_string())
        .unwrap_or_default()
}
