-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    steam_id VARCHAR(255) NOT NULL UNIQUE,
    username VARCHAR(255) NOT NULL,
    pf_url TEXT NOT NULL,
    avatar TEXT NOT NULL,
    persona_state INTEGER NOT NULL,
    visibility INTEGER NOT NULL,
    steam_created_at TIMESTAMPTZ NOT NULL,
    current_game VARCHAR(255),
    country VARCHAR(10),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);