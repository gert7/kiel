-- Your SQL goes here

CREATE TABLE switch_records (
    id SERIAL PRIMARY KEY,
    state INTEGER NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);
