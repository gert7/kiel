use rust_decimal::Decimal;
use serde::Deserialize;

use super::{MaskablePowerStrategy, PowerState, PriceChangeUnit};

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct PriceLimitStrategy {
    limit_mwh: Decimal,
}

impl MaskablePowerStrategy for PriceLimitStrategy {
    fn plan_day_masked<'a>(&self, mask: &'a [PriceChangeUnit]) -> Vec<PriceChangeUnit<'a>> {
        // println!("Running PriceLimitStrategy");
        mask.iter()
            .map(|pcu| match pcu.price {
                Some(price) => {
                    if price.total().0 > self.limit_mwh {
                        // println!("Price is too much {}, {}", price.total().0, self.limit_mwh);
                        PriceChangeUnit {
                            moment: price.moment,
                            price: pcu.price,
                            state: PowerState::Off,
                        }
                    } else {
                        // println!("Price ok");
                        *pcu
                    }
                }
                None => *pcu,
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use rust_decimal::Decimal;
    use rust_decimal_macros::dec;

    use crate::{sample_data::tests::sample_day_specified, strategy::{default::TariffStrategy, HourStrategy}};

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
        let base = TariffStrategy.plan_day(&sample_day);
        let strategy = PriceLimitStrategy {
            limit_mwh: dec!(150.0),
        };
        let result = strategy.plan_day_masked(&base);
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
