use chrono::DateTime;
use chrono_tz::Tz;
use serde::Deserialize;

use crate::{price_cell::PriceCell, price_matrix::DaySlice};

use super::{HourStrategy, PlannedChange, PowerState, PowerStrategy, PriceChangeUnit};

#[derive(Deserialize)]
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
    fn plan_day<'a>(&self, day_prices: &'a DaySlice) -> Vec<PriceChangeUnit<'a>> {
        day_prices
            .iter()
            .map(|price| PriceChangeUnit {
                price,
                change: PlannedChange {
                    moment: price.moment,
                    state: PowerState::On,
                },
            })
            .collect()
    }
}

#[derive(Deserialize)]
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
    fn plan_day<'a>(&self, day_prices: &'a DaySlice) -> Vec<PriceChangeUnit<'a>> {
        day_prices
            .iter()
            .map(|price| PriceChangeUnit {
                price,
                change: PlannedChange {
                    moment: price.moment,
                    state: PowerState::Off,
                },
            })
            .collect()
    }
}
