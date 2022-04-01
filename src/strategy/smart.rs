use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::Deserialize;

use crate::price_matrix::{DaySlice, PricePerMwh};

use super::{PlannedChange, PowerStrategy, PriceChangeUnit};

#[derive(Deserialize)]
pub struct SmartStrategy {
    hard_limit_mwh: Decimal,
    hour_budget: u32,
}

impl SmartStrategy {
    pub fn new() -> SmartStrategy {
        SmartStrategy {
            hard_limit_mwh: dec!(180.0),
            hour_budget: 9,
        }
    }
}

impl PowerStrategy for SmartStrategy {
    fn plan_day<'a>(&self, day_prices: &'a DaySlice) -> Vec<PriceChangeUnit<'a>> {
        let mut remaining_hours = self.hour_budget;

        for hour in day_prices {}
        vec![] // TODO: remove
    }
}
