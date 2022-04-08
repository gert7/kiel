use serde::Deserialize;

use super::{MaskablePowerStrategy, PriceChangeUnit};

#[derive(Clone, Copy, Deserialize)]
pub struct NoneStrategy;

impl MaskablePowerStrategy for NoneStrategy {
    fn plan_day_masked<'a>(&self, changes: &'a Vec<PriceChangeUnit>) -> Vec<PriceChangeUnit<'a>> {
        return changes.clone();
    }
}
