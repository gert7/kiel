use chrono::DateTime;
use chrono_tz::Tz;

use crate::{
    price_matrix::{DaySlice},
};

pub mod always;
pub mod default;
pub mod limit;
pub mod smart;

// pub use default::{DefaultStrategy, DefaultStrategyExclSunday};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PowerState {
    On,
    Off,
}

pub struct PlannedChange {
    pub moment: DateTime<Tz>,
    pub state: PowerState,
}

/// A power switching strategy simple enough
/// to only provide a power state for a single hour.
pub trait HourStrategy {
    fn plan_hour(&self, datetime: &DateTime<Tz>) -> PlannedChange;
}

/// A power switching strategy.
/// A strategy only has to consider the first 24 hours
/// of the provided day slice.
pub trait PowerStrategy {
    fn plan_day(&self, day_prices: &DaySlice) -> Vec<PlannedChange>;
}

/// A power switching strategy that can leave some
/// hours to be filled by some other provided [`HourStrategy`]
/// by setting the hour to None instead of Some.
pub trait MaskablePowerStrategy {
    fn plan_day_masked(&self, day_prices: &DaySlice, mask: &dyn HourStrategy) -> Vec<PlannedChange>;

    fn mask_description(&self) -> &'static str;
}
