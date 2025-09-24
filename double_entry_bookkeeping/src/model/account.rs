pub enum Account {
    Cash,
    AccountsReceivable,
    AccountsPayable,
    OwnerEquity,
    OwnerDrawings,
    OperatingExpense { expense_type: ExpenseType },
    Revenues,
    TaxPayable { tax_type: TaxType },
    TaxesPaid { tax_type: TaxType },
}

pub enum ExpenseType {
    Software,
    Hardware,
    Saas,
    Travel,
    Other,
}

pub enum TaxType {
    FederalIncomeTax,
    StateIncomeTax,
}
