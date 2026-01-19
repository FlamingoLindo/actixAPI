-- Add up migration script here
-- First, add the column as nullable
ALTER TABLE inventory_items ADD COLUMN classid VARCHAR;

-- Set a default value for existing rows (if any)
UPDATE inventory_items SET classid = 'unknown_' || id::text WHERE classid IS NULL;

-- Now make it NOT NULL
ALTER TABLE inventory_items ALTER COLUMN classid SET NOT NULL;

-- Create unique index
CREATE UNIQUE INDEX idx_inventory_items_unique ON inventory_items(inventory_id, classid);