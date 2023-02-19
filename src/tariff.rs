use chrono::{DateTime, Datelike, Timelike, Weekday};
use chrono_tz::Tz;

use crate::{constants::LOCAL_TZ, holidays::is_national_holiday};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Tariff {
    Night,
    Day,
}

impl Tariff {
    fn daytime_tariff(hour: u32) -> Tariff {
        if hour < 7 || hour >= 22 {
            Tariff::Night
        } else {
            Tariff::Day
        }
    }

    pub fn get_tariff(time: &DateTime<Tz>) -> Tariff {
        let time = time.with_timezone(&LOCAL_TZ);
        let day = time.weekday();
        if [Weekday::Sat, Weekday::Sun].contains(&day) || is_national_holiday(&time.date()) {
            Tariff::Night
        } else {
            Tariff::daytime_tariff(time.hour())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Date, TimeZone};
    use chrono_tz::{
        Europe::{Berlin, Tallinn},
        Tz,
    };

    /// Wednesday
    fn mmxxii_23_march() -> Date<Tz> {
        Tallinn.ymd(2022, 3, 23)
    }
    /// Saturday
    fn mmxxii_26_march() -> Date<Tz> {
        Tallinn.ymd(2022, 3, 26)
    }

    fn german_morning() -> DateTime<Tz> {
        Berlin.ymd(2022, 3, 23).and_hms(6, 13, 0)
    }

    fn german_evening() -> DateTime<Tz> {
        Berlin.ymd(2022, 3, 23).and_hms(21, 13, 0)
    }

    #[test]
    fn midnight_is_night() {
        let time = mmxxii_23_march().and_hms(0, 0, 0);
        assert!(Tariff::get_tariff(&time) == Tariff::Night);
    }

    #[test]
    fn wed_7() {
        let time = mmxxii_23_march().and_hms(7, 0, 0);
        assert!(Tariff::get_tariff(&time) == Tariff::Day);
    }

    #[test]
    fn wed_6_59() {
        let time = mmxxii_23_march().and_hms(6, 59, 59);
        assert!(Tariff::get_tariff(&time) == Tariff::Night);
    }

    #[test]
    fn wed_22() {
        let time = mmxxii_23_march().and_hms(22, 0, 0);
        assert!(Tariff::get_tariff(&time) == Tariff::Night);
    }

    #[test]
    fn wed_midday() {
        let time = mmxxii_23_march().and_hms(12, 0, 0);
        assert!(Tariff::get_tariff(&time) == Tariff::Day);
    }

    #[test]
    fn sat_midday() {
        let time = mmxxii_26_march().and_hms(12, 0, 0);
        assert!(Tariff::get_tariff(&time) == Tariff::Night);
    }

    #[test]
    fn sat_midnight() {
        let time = mmxxii_26_march().and_hms(0, 0, 0);
        assert!(Tariff::get_tariff(&time) == Tariff::Night);
    }

    #[test]
    fn german_morning_is_correct() {
        let time = german_morning();
        assert!(Tariff::get_tariff(&time) == Tariff::Day);
    }

    #[test]
    fn german_evening_is_correct() {
        let time = german_evening();
        assert!(Tariff::get_tariff(&time) == Tariff::Night);
    }
}
