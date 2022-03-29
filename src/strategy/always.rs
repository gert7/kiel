use chrono::DateTime;
use chrono_tz::Tz;

use crate::price_matrix::DaySlice;

use super::{HourStrategy, PlannedChange, PowerState, PowerStrategy};

pub struct AlwaysOnStrategy;

impl HourStrategy for AlwaysOnStrategy {
    fn plan_hour(&self, datetime: &DateTime<Tz>) -> PlannedChange {
        PlannedChange {
            moment: *datetime,
            state: PowerState::On,
        }
    }
}

impl PowerStrategy for AlwaysOnStrategy {
    fn plan_day(&self, day_prices: &DaySlice) -> Vec<PlannedChange> {
        day_prices.iter().map(|hour| {
            PlannedChange { moment: hour.moment, state: PowerState::On }
        }).collect()
    }
}

pub struct AlwaysOffStrategy;

impl HourStrategy for AlwaysOffStrategy {
    fn plan_hour(&self, datetime: &DateTime<Tz>) -> PlannedChange {
        PlannedChange {
            moment: *datetime,
            state: PowerState::Off,
        }
    }
}

impl PowerStrategy for AlwaysOffStrategy {
    fn plan_day(&self, day_prices: &DaySlice) -> Vec<PlannedChange> {
        day_prices.iter().map(|hour| {
            PlannedChange { moment: hour.moment, state: PowerState::Off }
        }).collect()
    }
}
