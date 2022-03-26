use chrono::{Date, DateTime, Duration};
use chrono_tz::Tz;

use crate::{
    price_matrix::{DateColumn, DaySlice, PriceCell},
    tariff::Tariff,
};

// mod default;

// pub use default::{DefaultStrategy, DefaultStrategyExclSunday};

#[derive(Debug, Clone)]
pub enum PowerState {
    On,
    Off,
}

pub struct PlannedChange {
    pub moment: DateTime<Tz>,
    pub state: PowerState,
}

pub fn add_almost_day(dt: DateTime<Tz>) -> DateTime<Tz> {
    let hours = 24 * 60 * 60;
    let minutes = 59 * 60;
    let seconds = 59;
    dt + Duration::seconds(hours + minutes + seconds)
}

pub fn truncate_to_24_hours(day_prices: &Vec<PriceCell>) -> Vec<PriceCell> {
    let mut sorted_prices = day_prices.clone();
    sorted_prices.sort_by(|a, b| a.moment.cmp(&b.moment));
    let first = sorted_prices.first();
    match first {
        Some(init) => {
            let day_cycle = add_almost_day(init.moment);
            let filtered_prices = sorted_prices
                .into_iter()
                .filter(|price| price.moment <= day_cycle);
            filtered_prices.collect()
        }
        None => sorted_prices,
    }
}

/// A power switching strategy.
/// A strategy only has to consider the first 24 hours
/// of the provided day slice.
pub trait PowerStrategy {
    fn plan_day(day_prices: &DaySlice) -> Vec<PlannedChange>;
}
