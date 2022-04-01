use chrono::DateTime;
use chrono_tz::Tz;

use crate::{
    price_matrix::{DaySlice}, price_cell::PriceCell,
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

#[derive(Clone, Copy)]
pub struct PlannedChange {
    pub moment: DateTime<Tz>,
    pub state: PowerState,
}

#[derive(Clone, Copy)]
pub struct PriceChangeUnit<'a> {
    pub price: &'a PriceCell,
    pub change: PlannedChange,
}

/// A power switching strategy simple enough
/// to only provide a power state for a single hour
/// with no price information provided.
pub trait HourStrategy {
    fn plan_hour(&self, datetime: &DateTime<Tz>) -> PlannedChange;
}

/// A power switching strategy.
/// A strategy only has to consider the first 24 hours
/// of the provided day slice.
pub trait PowerStrategy {
    fn plan_day<'a>(&self, day_prices: &'a DaySlice) -> Vec<PriceChangeUnit<'a>>;
}

/// A power switching strategy that accepts a set of
/// already-set price changes.
pub trait MaskablePowerStrategy {
    fn plan_day_masked<'a>(&self, changes: &'a Vec<PriceChangeUnit>) -> Vec<PriceChangeUnit<'a>>;
}
