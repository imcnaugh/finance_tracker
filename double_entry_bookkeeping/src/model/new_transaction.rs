use crate::model::account::Account;

pub struct NewTransaction {
    from_account: Account,
    to_account: Account,
    amount: i64,
    description: String,
}

impl NewTransaction {
    pub fn new(
        from_account: Account,
        to_account: Account,
        amount: i64,
        description: String,
    ) -> Self {
        Self {
            from_account,
            to_account,
            amount,
            description,
        }
    }
}
