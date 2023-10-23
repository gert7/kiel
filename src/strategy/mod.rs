use chrono::{Date, DateTime, Timelike};
use chrono_tz::Tz;
use eyre::{eyre, Result};
use now::DateTimeNow;

use crate::{constants::HOURS_OF_DAY, price_cell::PriceCell, price_matrix::DaySlice};

pub mod always;
pub mod default;
pub mod limit;
pub mod none;
pub mod power_state_model;
pub mod smart;

// pub use default::{DefaultStrategy, DefaultStrategyExclSunday};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PowerState {
    On,
    Off,
}

// #[derive(Clone, Copy, Debug)]
// pub struct PriceChangeUnit<'a> {
//     pub moment: DateTime<Tz>,
//     pub price: Option<&'a PriceCell>,
//     pub state: PowerState,
// }

#[derive(Clone, Copy, Debug)]
pub struct ChangeUnit<T> {
    pub moment: DateTime<Tz>,
    pub state: PowerState,
    pub price: T,
}

pub enum ChangeUnitEnum<'a> {
    Priced(ChangeUnit<&'a PriceCell>),
    Blank(ChangeUnit<()>)
}

pub type PriceChangeUnit<'a> = ChangeUnit<Option<&'a PriceCell>>;

impl<'a> PriceChangeUnit<'a> {
    pub fn clone_with_power_state(&self, state: PowerState) -> PriceChangeUnit<'a> {
        PriceChangeUnit {
            moment: self.moment,
            price: self.price,
            state,
        }
    }
}

/// A power switching strategy simple enough
/// to only provide a power state for a single hour
/// with no price information provided. Intended for
/// use as the base power pattern.
pub trait HourStrategy {
    fn plan_hour(&self, datetime: &DateTime<Tz>) -> PowerState;

    fn plan_day<'a>(&self, day_prices: &'a DaySlice) -> Vec<PriceChangeUnit<'a>>;

    /// Plans a day and fills any missing hours with the
    /// result that the strategy provides. Takes a day
    /// running from 0-23 hours.
    fn plan_day_full<'a>(
        &self,
        day_prices: &'a DaySlice,
        date: &DateTime<Tz>,
    ) -> Result<Vec<PriceChangeUnit<'a>>> {
        let mut vec = self.plan_day(day_prices);
        for hour in HOURS_OF_DAY {
            let hour: u32 = hour.into();
            let existing = vec
                .iter()
                .find(|pcu| pcu.moment.with_timezone(&date.timezone()).hour() == hour);
            if let None = existing {
                let moment = date.with_hour(hour).ok_or(eyre!("Unable to set hour."))?;
                let moment = moment.beginning_of_hour();
                let pcu = PriceChangeUnit {
                    moment,
                    price: None,
                    state: self.plan_hour(&moment),
                };
                vec.push(pcu);
            }
        }
        vec.sort_by(|a, b| a.moment.cmp(&b.moment));
        Ok(vec)
    }
}

/// A power switching strategy that accepts a set of
/// already-set price changes.
pub trait MaskablePowerStrategy {
    fn plan_day_masked<'a>(&self, changes: &'a Vec<PriceChangeUnit>) -> Vec<PriceChangeUnit<'a>>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Timelike};
    use rand::thread_rng;

    use crate::{
        constants::{MARKET_TZ, PLANNING_TZ},
        sample_data::tests::sample_day,
        strategy::{default::TariffStrategy, PowerState},
    };

    #[test]
    fn fills_gaps() {
        // let date = PLANNING_TZ.ymd(2022, 7, 14);
        let date = PLANNING_TZ.with_ymd_and_hms(2022, 7, 14, 0, 0, 0).earliest().unwrap();
        let day = sample_day(&date, 4, 12, &mut thread_rng()).unwrap();
        let filled = TariffStrategy.plan_day_full(&day, &date).unwrap();
        assert!(filled[0].moment.hour() == 0);
        assert!(filled[0].state == PowerState::On);
        assert!(filled[4].moment.hour() == 4);
        assert!(filled[4].state == PowerState::On);
        assert!(filled[6].moment.hour() == 6);
        assert!(filled[6].state == PowerState::Off);
        assert!(filled[12].moment.hour() == 12);
        assert!(filled[12].state == PowerState::Off);
        assert!(filled[20].moment.hour() == 20);
        assert!(filled[20].state == PowerState::Off);
        assert!(filled[23].moment.hour() == 23);
        assert!(filled[23].state == PowerState::On);
    }
}
