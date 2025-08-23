CREATE TABLE client (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    created_timestamp INT NOT NULL DEFAULT (unixepoch())
);

CREATE TABLE invoice (
    id TEXT PRIMARY KEY,
    client_id TEXT NOT NULL,
    draft_date INT NOT NULL,
    due_date INT,
    sent_date INT,
    paid_date INT,
    cancelled_date INT,
    FOREIGN KEY (client_id) REFERENCES client(id) ON DELETE CASCADE
);

CREATE TABLE line_item (
    id TEXT PRIMARY KEY,
    description TEXT NOT NULL,
    quantity REAL NOT NULL,
    unit_price_in_cents INTEGER NOT NULL,
    invoice_id TEXT NOT NULL,
    created_timestamp INT NOT NULL DEFAULT (unixepoch()),
    FOREIGN KEY (invoice_id) REFERENCES invoice(id) ON DELETE CASCADE
);

CREATE INDEX idx_invoice_client_id ON invoice(client_id);
CREATE INDEX idx_line_item_invoice_id ON line_item(invoice_id);

CREATE INDEX idx_invoice_status ON invoice(status);
