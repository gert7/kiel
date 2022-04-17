use std::cmp::Ordering;

use chrono::{DateTime, Timelike};
use chrono_tz::Tz;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::Deserialize;

use crate::{
    constants::LOCAL_TZ,
    price_cell::PriceCell,
    price_matrix::{DaySlice, PricePerMwh},
};

use super::{MaskablePowerStrategy, PowerState, PriceChangeUnit};

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct SmartStrategy {
    hour_budget: u8,
    morning_hours: u8,
    hard_limit_mwh: Decimal,
}

fn is_morning_hour(moment: &DateTime<Tz>) -> bool {
    let local_hour = moment.with_timezone(&LOCAL_TZ).hour();
    (0..7).contains(&local_hour)
}

fn hours_with_prices<'a>(
    changes: &'a Vec<PriceChangeUnit>,
) -> impl Iterator<Item = &'a PriceChangeUnit<'a>> {
    changes.iter().filter(|a| a.price.is_some())
}

fn prices_only<'a>(changes: &'a Vec<PriceChangeUnit>) -> impl Iterator<Item = &'a PriceCell> {
    changes.iter().filter_map(|pcu| pcu.price)
}

fn average_price(changes: &Vec<PriceChangeUnit>) -> Decimal {
    let prices = prices_only(&changes);
    let average = prices.fold((0, dec!(0)), |tu, price| (tu.0 + 1, tu.1 + price.total().0));
    average.1 / Decimal::from(average.0)
}

fn price_or_default(price_option: &Option<&PriceCell>, default: Decimal) -> Decimal {
    match price_option {
        Some(price_cell) => price_cell.total().0,
        None => default,
    }
}

fn sort_by_price(vec: &mut Vec<PriceChangeUnit>, default: Decimal) {
    vec.sort_by(|a, b| {
        price_or_default(&a.price, default).cmp(&price_or_default(&b.price, default))
    });
}

fn sort_by_price_refs(vec: &mut Vec<&PriceChangeUnit>, default: Decimal) {
    vec.sort_by(|a, b| {
        price_or_default(&a.price, default).cmp(&price_or_default(&b.price, default))
    });
}

impl MaskablePowerStrategy for SmartStrategy {
    fn plan_day_masked<'a>(&self, changes: &'a Vec<PriceChangeUnit>) -> Vec<PriceChangeUnit<'a>> {
        let morning_hour_count = self.morning_hours.clamp(0, 7);
        let count_of_hours_with_prices = hours_with_prices(&changes).count();
        if count_of_hours_with_prices < 20 {
            return changes.clone();
        }

        let ap = average_price(&changes);
        let mut sorted_by_price = changes.clone();
        sort_by_price(&mut sorted_by_price, ap);

        let mut result = vec![];

        let mut morning_sorted = sorted_by_price
            .iter()
            .filter(|pcu| is_morning_hour(&pcu.moment));
        let non_morning_sorted = sorted_by_price
            .iter()
            .filter(|pcu| !is_morning_hour(&pcu.moment));

        for _ in 0..morning_hour_count {
            println!("morning hour");
            let next = morning_sorted.next();
            if let Some(pcu) = next {
                println!("morning hour reserved: {}", pcu.moment);
                result.push(pcu.clone_with_power_state(PowerState::On));
            }
        }

        let mut remainder: Vec<_> = morning_sorted.chain(non_morning_sorted).collect();
        sort_by_price_refs(&mut remainder, ap);
        let mut remainder = remainder.into_iter();

        let remaining_on = self.hour_budget - morning_hour_count;

        for _ in 0..remaining_on {
            let next = remainder.next();
            if let Some(pcu) = next {
                result.push(pcu.clone_with_power_state(PowerState::On));
            }
        }

        while let Some(pcu) = remainder.next() {
            result.push(pcu.clone_with_power_state(PowerState::Off));
        }

        // HARD LIMIT
        for pcu_mut in result.iter_mut() {
            if let Some(price) = pcu_mut.price {
                if price.total().0 > self.hard_limit_mwh {
                    pcu_mut.state = PowerState::Off;
                }
            }
        }

        result.sort_by(|a, b| a.moment.cmp(&b.moment));

        result
    }
}

#[cfg(test)]
mod test {
    use rust_decimal::Decimal;
    use rust_decimal_macros::dec;

    use crate::{
        sample_data::sample_day_specified,
        strategy::{default::TariffStrategy, HourStrategy},
    };

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

    const SAMPLE_DAY_PRICES_FULL: [Decimal; 24] = [
        dec!(139.43), // 0
        dec!(134.30), // 1
        dec!(174.10), // 2
        dec!(190.39), // 3
        dec!(190.39), // 4
        dec!(150.39), // 5
        dec!(110.39), // 6
        dec!(33.39),  // 7
        dec!(49.33),  // 8
        dec!(59.30),  // 9
        dec!(100.10), // 10
        dec!(140.39), // 11
        dec!(159.39), // 12
        dec!(141.39), // 13
        dec!(42.39),  // 14
        dec!(33.39),  // 15
        dec!(120.33), // 16
        dec!(51.30),  // 17 // sorted number 7 by price ascending
        dec!(201.10), // 18
        dec!(41.39),  // 19
        dec!(58.39),  // 20
        dec!(95.39),  // 21
        dec!(79.39),  // 22
        dec!(12.39),  // 23
    ];

    #[test]
    fn gets_average() {
        let sample_day = sample_day_specified(&SAMPLE_DAY_PRICES, 0);
        let base = TariffStrategy.plan_day(&sample_day);
        let average = average_price(&base);
        println!("{}", average);
        assert!(average >= dec!(90.34));
        assert!(average < dec!(90.35));
    }

    #[test]
    fn sorts_prices() {
        let sample_day = sample_day_specified(&SAMPLE_DAY_PRICES_FULL, 0);
        let base = TariffStrategy.plan_day(&sample_day);
        for r in &base {
            println!("{:?}", r);
        }
        let strat = SmartStrategy {
            hour_budget: 7,
            morning_hours: 0,
            hard_limit_mwh: dec!(300.0),
        };
        let result = strat.plan_day_masked(&base);
        println!("Smart: \n");
        for r in &result {
            println!("{:?}", r);
        }
        assert!(result[0].state == PowerState::Off);
        assert!(result[1].state == PowerState::Off);
        assert!(result[2].state == PowerState::Off);
        assert!(result[3].state == PowerState::Off);
        assert!(result[4].state == PowerState::Off);
        assert!(result[5].state == PowerState::Off);
        assert!(result[6].state == PowerState::Off);
        assert!(result[7].state == PowerState::On);
        assert!(result[17].state == PowerState::On);
        assert!(result[18].state == PowerState::Off);
        assert!(result[19].state == PowerState::On);
        assert!(result[23].state == PowerState::On);
    }

    #[test]
    fn sorts_prices_with_morning() {
        let sample_day = sample_day_specified(&SAMPLE_DAY_PRICES_FULL, 0);
        let base = TariffStrategy.plan_day(&sample_day);
        for r in &base {
            println!("{:?}", r);
        }
        let strat = SmartStrategy {
            hour_budget: 7,
            morning_hours: 2,
            hard_limit_mwh: dec!(300.0),
        };
        let result = strat.plan_day_masked(&base);
        println!("Smart: \n");
        for r in &result {
            println!("{:?}", r);
        }
        assert!(result[0].state == PowerState::Off);
        assert!(result[1].state == PowerState::On);
        assert!(result[2].state == PowerState::Off);
        assert!(result[3].state == PowerState::Off);
        assert!(result[4].state == PowerState::Off);
        assert!(result[5].state == PowerState::Off);
        assert!(result[6].state == PowerState::Off);
        assert!(result[7].state == PowerState::On);
        assert!(result[17].state == PowerState::Off);
        assert!(result[18].state == PowerState::Off);
        assert!(result[19].state == PowerState::On);
        assert!(result[23].state == PowerState::On);
    }

    #[test]
    fn obeys_hard_limit() {
        let sample_day = sample_day_specified(&SAMPLE_DAY_PRICES_FULL, 0);
        let base = TariffStrategy.plan_day(&sample_day);
        for r in &base {
            println!("{:?}", r);
        }
        let strat = SmartStrategy {
            hour_budget: 7,
            morning_hours: 2,
            hard_limit_mwh: dec!(100.0),
        };
        let result = strat.plan_day_masked(&base);
        println!("Smart: \n");
        for r in &result {
            println!("{:?}", r);
        }
        assert!(result[0].state == PowerState::Off);
        assert!(result[1].state == PowerState::Off);
        assert!(result[2].state == PowerState::Off);
        assert!(result[3].state == PowerState::Off);
        assert!(result[4].state == PowerState::Off);
        assert!(result[5].state == PowerState::Off);
        assert!(result[6].state == PowerState::Off);
        assert!(result[7].state == PowerState::On);
        assert!(result[17].state == PowerState::Off);
        assert!(result[18].state == PowerState::Off);
        assert!(result[19].state == PowerState::On);
        assert!(result[23].state == PowerState::On);
    }
}
