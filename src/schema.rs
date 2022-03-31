table! {
    price_cells (id) {
        id -> Int4,
        price_mwh -> Numeric,
        moment_utc -> Timestamptz,
        tariff_mwh -> Nullable<Numeric>,
        market_hour -> Int2,
    }
}
