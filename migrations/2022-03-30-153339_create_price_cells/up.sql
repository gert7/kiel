-- Your SQL goes here

CREATE TABLE price_cells (
    id SERIAL PRIMARY KEY,
    price_mwh DECIMAL NOT NULL,
    moment_utc TIMESTAMPTZ NOT NULL,
    tariff_mwh DECIMAL,
    market_hour SMALLINT NOT NULL
);
