use chrono::DateTime;
use chrono_tz::Tz;

use crate::{
    price_matrix::{DaySlice},
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

/// A power switching strategy.
/// A strategy only has to consider the first 24 hours
/// of the provided day slice.
pub trait PowerStrategy {
    fn plan_day(day_prices: &DaySlice) -> Vec<PlannedChange>;
}
