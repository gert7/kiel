use rust_decimal_macros::dec;

use crate::price_matrix::{DaySlice, PricePerMwh};

use super::{PlannedChange, PowerStrategy};

pub struct SmartStrategy {
    hard_limit: PricePerMwh,
    hour_budget: u32,
    excl_sunday: bool,
}

impl SmartStrategy {
    pub fn new(excl_sunday: bool) -> SmartStrategy {
        SmartStrategy {
            hard_limit: PricePerMwh(dec!(180.0)),
            hour_budget: 9,
            excl_sunday,
        }
    }
}

impl PowerStrategy for SmartStrategy {
    fn plan_day(&self, day_prices: &DaySlice) -> Vec<PlannedChange> {
        let mut remaining_hours = self.hour_budget;

        for hour in day_prices {}
        vec![] // TODO: remove
    }
}
