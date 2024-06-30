use serde::Deserialize;

use super::{MaskablePowerStrategy, PriceChangeUnit};

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct NoneStrategy;

impl MaskablePowerStrategy for NoneStrategy {
    fn plan_day_masked<'a>(&self, changes: &'a [PriceChangeUnit]) -> Vec<PriceChangeUnit<'a>> {
        changes.to_vec()
    }
}
