-- Create some basic account types
INSERT INTO account_type (name) VALUES
('asset'),
('liability'),
('equity'),
('revenue'),
('expense');

-- Create some basic accounts
INSERT INTO account (name, account_type_id)
SELECT 'cash', id FROM account_type WHERE name = 'asset';

INSERT INTO account (name, account_type_id)
SELECT 'accounts receivable', id FROM account_type WHERE name = 'asset';

INSERT INTO account (name, account_type_id)
SELECT 'accounts payable', id FROM account_type WHERE name = 'liability';

INSERT INTO account (name, account_type_id)
SELECT 'tax payable', id FROM account_type WHERE name = 'liability';

INSERT INTO account (name, account_type_id)
SELECT 'owner equity', id FROM account_type WHERE name = 'equity';

INSERT INTO account (name, account_type_id)
SELECT 'owner drawings', id FROM account_type WHERE name = 'equity';

INSERT INTO account (name, account_type_id)
SELECT 'revenues', id FROM account_type WHERE name = 'revenue';

INSERT INTO account (name, account_type_id)
SELECT 'operating expense', id FROM account_type WHERE name = 'expense';

INSERT INTO account (name, account_type_id)
SELECT 'taxes paid', id FROM account_type WHERE name = 'expense';
