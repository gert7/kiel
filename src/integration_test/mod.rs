#[cfg(test)]
mod tests {
    use chrono::TimeZone;
    use rand::thread_rng;

    use crate::{sample_data, constants::MARKET_TZ, price_cell::{self, PriceCell}, database::establish_connection};

    #[test]
    fn integrate() {
        let connection = establish_connection();
        let start_date = MARKET_TZ.ymd(2022, 3, 13);
        let sample_day = sample_data::sample_day(&start_date, 0, 24, &mut thread_rng());
        PriceCell::insert_cells_into_database(&connection, &sample_day.0).unwrap();
    }
}