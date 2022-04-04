use chrono::{Date, DateTime, Duration};
use chrono_tz::Tz;
use color_eyre::eyre;
use diesel::PgConnection;
use rust_decimal::Decimal;
use serde::Deserialize;

use crate::price_cell::PriceCell;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct PricePerMwh(pub Decimal);

impl PricePerMwh {
    fn new(currency_per_mwh: Decimal) -> PricePerMwh {
        PricePerMwh(currency_per_mwh)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct CentsPerKwh(pub Decimal);

impl CentsPerKwh {
    fn new(cents_per_kwh: Decimal) -> CentsPerKwh {
        CentsPerKwh(cents_per_kwh)
    }
}

impl From<&PricePerMwh> for CentsPerKwh {
    fn from(e_per_mwh: &PricePerMwh) -> Self {
        CentsPerKwh(e_per_mwh.0 / Decimal::from(10i32))
    }
}

impl From<&CentsPerKwh> for PricePerMwh {
    fn from(c_per_kwh: &CentsPerKwh) -> Self {
        PricePerMwh(c_per_kwh.0 * Decimal::from(10i32))
    }
}

#[derive(Clone, Debug)]
pub struct DateColumn {
    pub date: Date<Tz>,
    pub cells: DaySlice,
}

pub type PriceMatrix = Vec<Option<DateColumn>>;

#[derive(Clone, Debug)]
pub struct DaySlice(pub Vec<PriceCell>);

// #[derive(Clone, Debug)]
// pub struct DayNightSlice {
//     pub split_hour: u32,
//     pub prices: Vec<PriceCell>,
// }

pub fn add_almost_day(dt: &DateTime<Tz>) -> DateTime<Tz> {
    let hours = Duration::hours(23);
    let minutes = Duration::minutes(59);
    let seconds = Duration::seconds(59);
    *dt + hours + minutes + seconds
}

pub fn truncate_to_24_hours(day_prices: &DaySlice) -> DaySlice {
    let mut sorted_prices = day_prices.0.clone();
    sorted_prices.sort_by(|a, b| a.moment.cmp(&b.moment));
    let first = sorted_prices.first();
    let vec = match first {
        Some(init) => {
            let day_cycle = add_almost_day(&init.moment);
            let filtered_prices = sorted_prices
                .into_iter()
                .filter(|price| price.moment <= day_cycle);
            filtered_prices.collect()
        }
        None => sorted_prices,
    };
    DaySlice(vec)
}

pub fn insert_matrix_to_database(
    connection: &PgConnection,
    date_matrix: &PriceMatrix,
) -> eyre::Result<()> {
    let date_matrix = date_matrix
        .iter()
        .filter(|o| o.is_some())
        .map(|o| o.as_ref().unwrap());

    for date in date_matrix {
        for price in &date.cells.0 {
            price.insert_cell_into_database(&connection)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;
    use rand::thread_rng;

    use crate::{constants::MARKET_TZ, sample_data::sample_day};

    use super::*;

    #[test]
    fn converts_to_cpkh() {
        let mwh = PricePerMwh(Decimal::new(14899, 2));
        let kph = CentsPerKwh::from(&mwh);
        assert!(kph.0 == Decimal::new(14899, 3));
    }

    #[test]
    fn converts_to_mh() {
        let kph = CentsPerKwh(Decimal::new(948, 2));
        let mwh = PricePerMwh::from(&kph);
        assert!(mwh.0 == Decimal::new(948, 1));
    }

    #[test]
    fn adds_almost_day() {
        let date1 = MARKET_TZ.ymd(2022, 3, 3).and_hms(0, 0, 0);
        let added = add_almost_day(&date1);
        assert!(added == MARKET_TZ.ymd(2022, 3, 3).and_hms(23, 59, 59));
    }

    #[test]
    fn truncates_properly() {
        let date1 = MARKET_TZ.ymd(2022, 3, 3);
        let sample_day = sample_day(&date1, 16, 30, &mut thread_rng());
        assert!(sample_day.0.len() == 30);
        let truncated = truncate_to_24_hours(&sample_day);
        assert!(truncated.0.len() == 24);
    }
}
