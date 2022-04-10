table! {
    convar_ints (id) {
        id -> Int4,
        key -> Text,
        value -> Int4,
        created_at -> Timestamptz,
    }
}

table! {
    convar_strings (id) {
        id -> Int4,
        key -> Text,
        value -> Text,
        created_at -> Timestamptz,
    }
}

table! {
    day_configurations (id) {
        id -> Int4,
        toml -> Text,
        known_broken -> Bool,
        tried -> Bool,
        created_at -> Timestamptz,
    }
}

table! {
    power_states (id) {
        id -> Int4,
        moment_utc -> Timestamptz,
        state -> Int4,
        configuration_id -> Nullable<Int4>,
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
    convar_ints,
    convar_strings,
    day_configurations,
    power_states,
    price_cells,
    switch_records,
);
