use chrono::{DateTime, Local, Datelike, Weekday, Timelike};
use chrono_tz::{Tz, Europe::Tallinn};

#[derive(Debug, PartialEq)]
pub enum Tariff {
    Night,
    Day,
}

impl Tariff {
pub fn get_tariff(time: &DateTime<Tz>) -> Tariff {
    let time = time.with_timezone(&Tallinn);
    let day = time.weekday();
    if [Weekday::Sat, Weekday::Sun].contains(&day) {
        Tariff::Night
    } else {
        let hour = time.hour();
        if hour < 7 || hour >= 22 {
            Tariff::Night
        } else {
            Tariff::Day
        }
    }
}
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Date};
    use chrono_tz::{Europe::Tallinn, Tz};

    /// Wednesday
    fn mmxxii_23_march() -> Date<Tz> { Tallinn.ymd(2022, 3, 23) }
    /// Saturday
    fn mmxxii_26_march() -> Date<Tz> { Tallinn.ymd(2022, 3, 26) }

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
}
