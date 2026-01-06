-- Add up migration script here
ALTER TABLE users ADD COLUMN IF NOT EXISTS gameid VARCHAR(50);