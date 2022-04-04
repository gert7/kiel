-- Your SQL goes here

CREATE TABLE power_states (
    id SERIAL PRIMARY KEY,
    moment_utc TIMESTAMPTZ NOT NULL,
    state INTEGER NOT NULL,
    configuration_id INTEGER,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);
