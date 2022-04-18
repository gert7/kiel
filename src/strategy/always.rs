use chrono::DateTime;
use chrono_tz::Tz;
use serde::Deserialize;

use crate::price_matrix::DaySlice;

use super::{HourStrategy, PowerState, PriceChangeUnit};

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct AlwaysOnStrategy;

impl HourStrategy for AlwaysOnStrategy {
    fn plan_hour(&self, datetime: &DateTime<Tz>) -> PowerState {
        PowerState::On
    }
    fn plan_day<'a>(&self, day_prices: &'a DaySlice) -> Vec<PriceChangeUnit<'a>> {
        day_prices
            .0
            .iter()
            .map(|price| PriceChangeUnit {
                price: Some(price),
                state: PowerState::On,
                moment: price.moment,
            })
            .collect()
    }
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct AlwaysOffStrategy;

impl HourStrategy for AlwaysOffStrategy {
    fn plan_hour(&self, datetime: &DateTime<Tz>) -> PowerState {
        PowerState::Off
    }
    fn plan_day<'a>(&self, day_prices: &'a DaySlice) -> Vec<PriceChangeUnit<'a>> {
        day_prices
            .0
            .iter()
            .map(|price| PriceChangeUnit {
                price: Some(price),
                state: PowerState::Off,
                moment: price.moment,
            })
            .collect()
    }
}
