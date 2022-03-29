use crate::price_matrix::{DaySlice, PricePerMwh};

use super::{PlannedChange, PowerStrategy};

pub struct SmartStrategy {
    hard_limit: PricePerMwh,
    hour_budget: u32,
    excl_sunday: bool,
}

impl PowerStrategy for SmartStrategy {
    fn plan_day(&self, day_prices: &DaySlice) -> Vec<PlannedChange> {
        let mut remaining_hours = self.hour_budget;

        for hour in day_prices {
            
        }
    }
}
