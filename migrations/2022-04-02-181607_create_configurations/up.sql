-- Your SQL goes here

CREATE TABLE day_configurations (
    id SERIAL PRIMARY KEY,
    toml TEXT NOT NULL,
    known_broken BOOLEAN NOT NULL,
    tried BOOLEAN NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);
