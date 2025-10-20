use double_entry_bookkeeping::model::Account;

pub struct AccountDisplay {
    id: String,
    name: String,
    balance: String,
}

impl From<&Account> for AccountDisplay {
    fn from(value: &Account) -> Self {
        todo!()
    }
}
