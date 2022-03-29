use rust_decimal::Decimal;

use crate::price_matrix::{DaySlice, PricePerMwh};

use super::{HourStrategy, MaskablePowerStrategy, PlannedChange, PowerState};

pub struct PriceLimitStrategy {
    limit: PricePerMwh,
}

impl PriceLimitStrategy {
    pub fn new(limit: PricePerMwh) -> Self {
        PriceLimitStrategy { limit }
    }
}

impl MaskablePowerStrategy for PriceLimitStrategy {
    fn plan_day_masked(
        &self,
        day_prices: &DaySlice,
        mask: &dyn HourStrategy,
    ) -> Vec<PlannedChange> {
        day_prices
            .iter()
            .map(|price| {
                if price.price > self.limit {
                    PlannedChange {
                        moment: price.moment,
                        state: PowerState::Off,
                    }
                } else {
                    mask.plan_hour(&price.moment)
                }
            })
            .collect()
    }

    fn mask_description(&self) -> &'static str {
        "fallback"
    }
}

#[cfg(test)]
mod test {
    use rust_decimal_macros::dec;

    use crate::{sample_data::sample_day_specified, strategy::default::DefaultStrategy};

    use super::*;
    const SAMPLE_DAY_PRICES: [Decimal; 8] = [
        dec!(39.43),  // 0
        dec!(134.30), // 1
        dec!(74.10),  // 2
        dec!(190.39), // 3
        dec!(90.39),  // 4
        dec!(150.39), // 5
        dec!(10.39),  // 6
        dec!(33.39),  // 7
    ];

    #[test]
    fn hits_limit() {
        let sample_day = sample_day_specified(&SAMPLE_DAY_PRICES, 0);
        let mask = DefaultStrategy;
        let result =
            PriceLimitStrategy::new(PricePerMwh(dec!(150.0))).plan_day_masked(&sample_day, &mask);
        assert!(result[0].state == PowerState::On);
        assert!(result[1].state == PowerState::On);
        assert!(result[2].state == PowerState::On);
        assert!(result[3].state == PowerState::Off);
        assert!(result[4].state == PowerState::On);
        assert!(result[5].state == PowerState::Off);
        assert!(result[6].state == PowerState::Off);
        assert!(result[7].state == PowerState::Off);
    }
}
