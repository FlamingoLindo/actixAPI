-- Add down migration script here
-- Revert icon column back to original length
ALTER TABLE inventory_items ALTER COLUMN icon TYPE VARCHAR(255);
