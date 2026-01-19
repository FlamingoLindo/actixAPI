-- Add up migration script here
-- Increase icon column length to handle longer Steam URLs
ALTER TABLE inventory_items ALTER COLUMN icon TYPE VARCHAR(500);
