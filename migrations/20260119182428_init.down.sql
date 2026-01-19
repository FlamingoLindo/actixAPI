-- Add down migration script here
DROP INDEX IF EXISTS idx_inventory_items_unique;
ALTER TABLE inventory_items DROP COLUMN IF EXISTS classid;