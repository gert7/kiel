use chrono::Date;
use chrono_tz::Tz;

use crate::{
    price_matrix::{DaySlice, PriceCell},
    tariff::Tariff,
};

use super::{PlannedChange, PowerState, PowerStrategy};

pub struct DefaultStrategy;

impl DefaultStrategy {
    fn tariff_to_power_state(tariff: &Tariff) -> PowerState {
        match tariff {
            Tariff::Night => PowerState::On,
            Tariff::Day => PowerState::Off,
        }
    }
}

impl PowerStrategy for DefaultStrategy {
    fn plan_day(day_prices: &DaySlice) -> Vec<PlannedChange> {
        let mut result = Vec::with_capacity(24);
        for hour in 0u32..24 {
            let datetime = date.and_hms(hour, 0, 0);
            let tariff = Tariff::get_tariff(&datetime);
            result.push(PlannedChange {
                moment: datetime,
                state: DefaultStrategy::tariff_to_power_state(&tariff),
            });
        }
        result
    }
}

pub struct DefaultStrategyExclSunday;

impl PowerStrategy for DefaultStrategyExclSunday {
    fn plan_day(day_prices: &DaySlice) -> Vec<PlannedChange> {
        let mut result = Vec::with_capacity(24);
        for hour in 0u32..24 {
            let datetime = date.and_hms(hour, 0, 0);
            let tariff = Tariff::get_tariff_excl_sunday(&datetime);
            result.push(PlannedChange {
                moment: datetime,
                state: DefaultStrategy::tariff_to_power_state(&tariff),
            });
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;
    use chrono_tz::Europe::Tallinn;

    use super::*;

    /// Wednesday
    fn mmxxii_23_march() -> Date<Tz> {
        Tallinn.ymd(2022, 3, 23)
    }
    /// Saturday
    fn mmxxii_26_march() -> Date<Tz> {
        Tallinn.ymd(2022, 3, 26)
    }

    #[test]
    fn makes_default_strategy() {
        let date = mmxxii_23_march();
        let planned_day = DefaultStrategy::plan_day(None);
        for change in planned_day {
            println!("{:?} {:?}", change.moment, change.state);
        }
    }
}
