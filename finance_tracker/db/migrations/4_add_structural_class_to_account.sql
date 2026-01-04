-- Add structural_class column to account table
-- This tracks which side of the accounting equation (Assets = Liabilities + Equity)
-- each account belongs to.
--
-- Note: Revenue and expense accounts are classified as EQUITY because they are
-- temporary equity accounts that explain how equity changed during a period.
-- They get closed into equity at period end.

ALTER TABLE account ADD COLUMN structural_class TEXT NOT NULL
CHECK (structural_class IN ('ASSET', 'LIABILITY', 'EQUITY'))
DEFAULT 'ASSET';

-- Update existing accounts with their correct structural_class
-- Asset accounts stay as ASSET
UPDATE account
SET structural_class = 'ASSET'
WHERE account_type_id IN (SELECT id FROM account_type WHERE name = 'asset');

-- Liability accounts become LIABILITY
UPDATE account
SET structural_class = 'LIABILITY'
WHERE account_type_id IN (SELECT id FROM account_type WHERE name = 'liability');

-- Equity accounts become EQUITY
UPDATE account
SET structural_class = 'EQUITY'
WHERE account_type_id IN (SELECT id FROM account_type WHERE name = 'equity');

-- Revenue accounts are EQUITY (temporary equity increase)
UPDATE account
SET structural_class = 'EQUITY'
WHERE account_type_id IN (SELECT id FROM account_type WHERE name = 'revenue');

-- Expense accounts are EQUITY (temporary equity decrease)
UPDATE account
SET structural_class = 'EQUITY'
WHERE account_type_id IN (SELECT id FROM account_type WHERE name = 'expense');

-- Remove the default constraint now that all existing rows are updated
-- SQLite doesn't support DROP DEFAULT, so we need to recreate the table if needed
-- For now, the DEFAULT 'ASSET' is harmless as new accounts should specify structural_class explicitly
