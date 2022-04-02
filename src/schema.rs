table! {
    day_configurations (id) {
        id -> Int4,
        toml -> Text,
        known_broken -> Bool,
        created_at -> Timestamptz,
    }
}

table! {
    price_cells (id) {
        id -> Int4,
        price_mwh -> Numeric,
        moment_utc -> Timestamptz,
        tariff_mwh -> Nullable<Numeric>,
        market_hour -> Int2,
        created_at -> Timestamptz,
    }
}

table! {
    switch_records (id) {
        id -> Int4,
        state -> Int4,
        created_at -> Timestamptz,
    }
}

allow_tables_to_appear_in_same_query!(
    day_configurations,
    price_cells,
    switch_records,
);
