-- Add up migration script here

CREATE TABLE IF NOT EXISTS games (
    id UUID PRIMARY KEY,
    appid VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    short_description TEXT,
    header_image VARCHAR,
    screenshots TEXT[]
);

CREATE TABLE user_games (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    game_id UUID NOT NULL REFERENCES games(id) ON DELETE CASCADE,
    UNIQUE(user_id, game_id)
);