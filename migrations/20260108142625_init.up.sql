-- Add up migration script here
ALTER TABLE games ALTER COLUMN id SET DEFAULT uuid_generate_v4();
