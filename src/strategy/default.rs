use chrono::DateTime;
use chrono_tz::Tz;

use crate::{price_matrix::DaySlice, tariff::Tariff};
use super::{HourStrategy, PlannedChange, PowerState, PowerStrategy, PriceChangeUnit};

pub struct DefaultStrategy;

impl DefaultStrategy {
    fn tariff_to_power_state(tariff: Tariff) -> PowerState {
        match tariff {
            Tariff::Night => PowerState::On,
            Tariff::Day => PowerState::Off,
        }
    }
}

impl HourStrategy for DefaultStrategy {
    fn plan_hour(&self, datetime: &DateTime<Tz>) -> PlannedChange {
        let tariff = Tariff::get_tariff(&datetime);
        let state = DefaultStrategy::tariff_to_power_state(tariff);
        PlannedChange {
            moment: *datetime,
            state,
        }
    }
}

impl PowerStrategy for DefaultStrategy {
    fn plan_day<'a>(&self, day_prices: &'a DaySlice) -> Vec<PriceChangeUnit<'a>> {
        day_prices
            .iter()
            .map(|price| PriceChangeUnit {price, change: self.plan_hour(&price.moment)})
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Date};
    use chrono_tz::Europe::Tallinn;
    use rand::thread_rng;

    use crate::sample_data::sample_day;

    use super::*;

    /// Wednesday
    fn mmxxii_23_march() -> Date<Tz> {
        Tallinn.ymd(2022, 3, 23)
    }

    /// Saturday
    fn mmxxii_19_march() -> Date<Tz> {
        Tallinn.ymd(2022, 3, 19)
    }

    #[test]
    fn makes_default_strategy() {
        let date = mmxxii_23_march();
        let sample_day = sample_day(&date, 14, 24, &mut thread_rng());
        let planned_day = DefaultStrategy.plan_day(&sample_day);
        assert!(planned_day[0].change.moment == date.and_hms(14, 0, 0));
        assert!(planned_day[1].change.moment == date.and_hms(15, 0, 0));

        assert!(planned_day[0].change.state == PowerState::Off);
        assert!(planned_day[1].change.state == PowerState::Off);
        assert!(planned_day[10].change.state == PowerState::On);
        assert!(planned_day[23].change.state == PowerState::Off);
    }

    #[test]
    fn makes_default_strategy_on_saturday() {
        let date = mmxxii_19_march();
        let sample_day = sample_day(&date, 14, 24, &mut thread_rng());
        let planned_day = DefaultStrategy.plan_day(&sample_day);
        assert!(planned_day[0].change.state == PowerState::On);
        assert!(planned_day[1].change.state == PowerState::On);
        assert!(planned_day[10].change.state == PowerState::On);
        assert!(planned_day[23].change.state == PowerState::On);
    }

}
