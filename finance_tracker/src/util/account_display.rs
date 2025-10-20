use comfy_table::Table;
use comfy_table::presets::UTF8_FULL;
use double_entry_bookkeeping::model::Account;

pub struct AccountDisplay {
    id: String,
    name: String,
    balance: String,
    account_type: String,
    normal_balance: String,
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
}
