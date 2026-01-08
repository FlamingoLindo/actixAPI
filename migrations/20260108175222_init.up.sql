-- Add up migration script here
ALTER TABLE user_games ALTER COLUMN id SET DEFAULT uuid_generate_v4();
