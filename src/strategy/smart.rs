use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::Deserialize;

use crate::price_matrix::{DaySlice, PricePerMwh};

use super::{PriceChangeUnit, MaskablePowerStrategy};

#[derive(Clone, Copy, Deserialize)]
pub struct SmartStrategy {
    hour_budget: u32,
}

impl MaskablePowerStrategy for SmartStrategy {
    fn plan_day_masked<'a>(&self, changes: &'a Vec<PriceChangeUnit>) -> Vec<PriceChangeUnit<'a>> {
        let mut remaining_hours = self.hour_budget;

        for hour in changes {}
        vec![] // TODO: remove
    }
}
