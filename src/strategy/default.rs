use chrono::DateTime;
use chrono_tz::Tz;
use serde::Deserialize;

use super::{HourStrategy, PowerState, PriceChangeUnit};
use crate::{price_matrix::DaySlice, tariff::Tariff};

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct TariffStrategy;

impl TariffStrategy {
    fn tariff_to_power_state(tariff: Tariff) -> PowerState {
        match tariff {
            Tariff::Night => PowerState::On,
            Tariff::Day => PowerState::Off,
        }
    }
}

impl HourStrategy for TariffStrategy {
    fn plan_hour(&self, datetime: &DateTime<Tz>) -> PowerState {
        let tariff = Tariff::get_tariff(&datetime);
        TariffStrategy::tariff_to_power_state(tariff)
    }
    fn plan_day<'a>(&self, day_prices: &'a DaySlice) -> Vec<PriceChangeUnit<'a>> {
        day_prices.0
            .iter()
            .map(|price| PriceChangeUnit {
                price: Some(price),
                state: self.plan_hour(&price.moment),
                moment: price.moment,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Timelike};
    use chrono_tz::Europe::Tallinn;
    use now::DateTimeNow;
    use rand::thread_rng;

    use crate::sample_data::tests::sample_day;

    use super::*;

    /// Wednesday
    fn mmxxii_23_march() -> DateTime<Tz> {
        Tallinn.with_ymd_and_hms(2022, 3, 23, 0, 0, 0).earliest().unwrap()
    }

    /// Saturday
    fn mmxxii_19_march() -> DateTime<Tz> {
        Tallinn.with_ymd_and_hms(2022, 3, 19, 0, 0, 0).earliest().unwrap()
    }

    #[test]
    fn makes_default_strategy() {
        let date = mmxxii_23_march();
        let sample_day = sample_day(&date, 14, 24, &mut thread_rng()).unwrap();
        let planned_day = TariffStrategy.plan_day(&sample_day);
        let fourteen = date.with_hour(14).unwrap().beginning_of_hour();
        assert!(planned_day[0].moment == fourteen);
        let fifteen = date.with_hour(15).unwrap().beginning_of_hour();
        assert!(planned_day[1].moment == fifteen);

        assert!(planned_day[0].state == PowerState::Off);
        assert!(planned_day[1].state == PowerState::Off);
        assert!(planned_day[10].state == PowerState::On);
        assert!(planned_day[23].state == PowerState::Off);
    }

    #[test]
    fn makes_default_strategy_on_saturday() {
        let date = mmxxii_19_march();
        let sample_day = sample_day(&date, 14, 24, &mut thread_rng()).unwrap();
        let planned_day = TariffStrategy.plan_day(&sample_day);
        assert!(planned_day[0].state == PowerState::On);
        assert!(planned_day[1].state == PowerState::On);
        assert!(planned_day[10].state == PowerState::On);
        assert!(planned_day[23].state == PowerState::On);
    }
}
