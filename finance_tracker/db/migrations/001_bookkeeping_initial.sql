CREATE TABLE account_type (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    created_timestamp INT NOT NULL DEFAULT (unixepoch())
);

CREATE TABLE account (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    account_type_id INTEGER NOT NULL,
    created_timestamp INT NOT NULL DEFAULT (unixepoch()),
    FOREIGN KEY (account_type_id) REFERENCES account_type(id)
);

CREATE TABLE journal_entry(
     id INTEGER PRIMARY KEY AUTOINCREMENT,
     description TEXT NOT NULL,
     created_timestamp INT NOT NULL DEFAULT (unixepoch())
);

CREATE TABLE transaction(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    account_id INTEGER NOT NULL,
    journal_entry_id INTEGER NOT NULL,
    amount_in_cents INT NOT NULL,
    is_debit BOOLEAN NOT NULL,
    created_timestamp INT NOT NULL DEFAULT (unixepoch()),
    FOREIGN KEY (account_id) REFERENCES account(id),
    FOREIGN KEY (journal_entry_id) REFERENCES journal_entry(id)
);

CREATE INDEX idx_account_type ON account (account_type_id);
CREATE INDEX idx_transaction_account_id ON transaction (account_id);
CREATE INDEX idx_transaction_journal_entry_id ON transaction (journal_entry_id);
