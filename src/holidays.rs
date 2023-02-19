use bdays::easter::{easter_naive_date, EasterError};
use chrono::{Date, Datelike, Duration};
use chrono_tz::Tz;
use lazy_static::lazy_static;

use crate::constants::MARKET_TZ;

struct MonthDay {
    month: u32,
    day: u32,
}

// month-day
const FIXED_NATL_HOLIDAYS: [MonthDay; 8] = [
    // uusaasta
    MonthDay { month: 1, day: 1 },
    // kevadpüha
    MonthDay { month: 5, day: 1 },
    // võidupüha
    MonthDay { month: 6, day: 23 },
    // jaanipäev
    MonthDay { month: 6, day: 24 },
    // taasiseseisvumispäev
    MonthDay { month: 8, day: 20 },
    // jõululaupäev
    MonthDay { month: 12, day: 24 },
    // 1. jõulupäuha
    MonthDay { month: 12, day: 25 },
    // 2. jõulupäuha
    MonthDay { month: 12, day: 26 },
];

fn precompute_easter_dates() -> Result<[MonthDay; 6], EasterError> {
    let now = chrono::Local::now().with_timezone(&MARKET_TZ);
    let e1 = easter_naive_date(now.year())?;
    let e2 = easter_naive_date(now.year() + 1)?;
    let gf1 = e1 - Duration::days(2);
    let gf2 = e2 - Duration::days(2);
    let pc1 = e1 + Duration::days(49);
    let pc2 = e2 + Duration::days(49);
    Ok([
        MonthDay {
            month: e1.month(),
            day: e1.day(),
        },
        MonthDay {
            month: e2.month(),
            day: e2.day(),
        },
        MonthDay {
            month: gf1.month(),
            day: gf1.day(),
        },
        MonthDay {
            month: gf2.month(),
            day: gf2.day(),
        },
        MonthDay {
            month: pc1.month(),
            day: pc1.day(),
        },
        MonthDay {
            month: pc2.month(),
            day: pc2.day(),
        },
    ])
}

lazy_static! {
    static ref EASTER_DATES: Result<[MonthDay; 6], EasterError> = precompute_easter_dates();
}

fn is_same_date<D1: Datelike, D2: Datelike>(d1: &D1, d2: &D2) -> bool {
    d1.month() == d2.month() && d1.day() == d2.day()
}

fn is_fixed_national_holiday(date: &Date<Tz>) -> bool {
    FIXED_NATL_HOLIDAYS
        .iter()
        .any(|hd| hd.month == date.month() && hd.day == date.day())
}

fn is_easter_or_good_friday(date: &Date<Tz>) -> bool {
    let easter = easter_naive_date(date.year());
    if let Ok(easter) = easter {
        let good_friday = easter - Duration::days(2);
        let pentecost = easter + Duration::days(49);
        is_same_date(&easter, date)
            || is_same_date(&good_friday, date)
            || is_same_date(&pentecost, date)
    } else {
        false
    }
}

fn is_easter_related_date(date: &Date<Tz>) -> bool {
    if let Ok(easter_holidays) = &*EASTER_DATES {
        easter_holidays
            .iter()
            .any(|hd| hd.month == date.month() && hd.day == date.day())
    } else {
        false
    }
}

fn is_moving_national_holiday(date: &Date<Tz>) -> bool {
    is_easter_related_date(date)
}

pub fn is_national_holiday(date: &Date<Tz>) -> bool {
    is_fixed_national_holiday(date) || is_moving_national_holiday(date)
}

#[cfg(test)]
mod test {
    use chrono::TimeZone;
    use chrono_tz::Europe::Tallinn;

    use super::*;

    #[test]
    fn checks_easter() {
        let easter_2021 = Tallinn.ymd(2021, 4, 4);
        assert!(is_easter_or_good_friday(&easter_2021));
        let not_easter_2021 = Tallinn.ymd(2021, 4, 5);
        assert!(!is_easter_or_good_friday(&not_easter_2021));
        let easter_2022 = Tallinn.ymd(2022, 4, 17);
        assert!(is_easter_or_good_friday(&easter_2022));
        let not_easter_2022 = Tallinn.ymd(2022, 4, 16);
        assert!(!is_easter_or_good_friday(&not_easter_2022));
        let easter_2023 = Tallinn.ymd(2023, 4, 9);
        assert!(is_easter_or_good_friday(&easter_2023));
        let easter_2024 = Tallinn.ymd(2024, 3, 31);
        assert!(is_easter_or_good_friday(&easter_2024));
    }

    #[test]
    fn checks_good_friday() {
        let good_friday_2021 = Tallinn.ymd(2021, 4, 2);
        assert!(is_easter_or_good_friday(&good_friday_2021));
        let not_good_friday_2021 = Tallinn.ymd(2021, 4, 1);
        assert!(!is_easter_or_good_friday(&not_good_friday_2021));
        let good_friday_2022 = Tallinn.ymd(2022, 4, 15);
        assert!(is_easter_or_good_friday(&good_friday_2022));
        let not_good_friday_2022 = Tallinn.ymd(2022, 4, 16);
        assert!(!is_easter_or_good_friday(&not_good_friday_2022));
        let good_friday_2023 = Tallinn.ymd(2023, 4, 7);
        assert!(is_easter_or_good_friday(&good_friday_2023));
        let good_friday_2024 = Tallinn.ymd(2024, 3, 29);
        assert!(is_easter_or_good_friday(&good_friday_2024));
    }

    #[test]
    fn checks_pentecost() {
        let pentecost_2021 = Tallinn.ymd(2021, 5, 23);
        assert!(is_easter_or_good_friday(&pentecost_2021));
        let not_pentecost_2021 = Tallinn.ymd(2021, 5, 24);
        assert!(!is_easter_or_good_friday(&not_pentecost_2021));
        let pentecost_2022 = Tallinn.ymd(2022, 6, 5);
        assert!(is_easter_or_good_friday(&pentecost_2022));
        let not_pentecost_2022 = Tallinn.ymd(2022, 6, 12);
        assert!(!is_easter_or_good_friday(&not_pentecost_2022));
        let pentecost_2023 = Tallinn.ymd(2023, 5, 28);
        assert!(is_easter_or_good_friday(&pentecost_2023));
        let not_pentecost_2023 = Tallinn.ymd(2023, 5, 29);
        assert!(!is_easter_or_good_friday(&not_pentecost_2023));
        let pentecost_2024 = Tallinn.ymd(2024, 5, 19);
        assert!(is_easter_or_good_friday(&pentecost_2024));
        let not_pentecost_2024 = Tallinn.ymd(2024, 5, 20);
        assert!(!is_easter_or_good_friday(&not_pentecost_2024));
    }

    #[test]
    fn checks_fixed_holidays() {
        let new_years = Tallinn.ymd(2023, 1, 1);
        assert!(is_national_holiday(&new_years));
        let may_day = Tallinn.ymd(2024, 5, 1);
        assert!(is_national_holiday(&may_day));
        let v_day = Tallinn.ymd(2025, 6, 23);
        assert!(is_national_holiday(&v_day));
        let j_day = Tallinn.ymd(2026, 6, 24);
        assert!(is_national_holiday(&j_day));
        let re_day = Tallinn.ymd(2027, 8, 20);
        assert!(is_national_holiday(&re_day));
        let christmas = Tallinn.ymd(2028, 12, 24);
        assert!(is_national_holiday(&christmas));
        let christmas_1 = Tallinn.ymd(2029, 12, 25);
        assert!(is_national_holiday(&christmas_1));
        let christmas_2 = Tallinn.ymd(2030, 12, 26);
        assert!(is_national_holiday(&christmas_2));
        let old_years = Tallinn.ymd(2023, 12, 31);
        assert!(!is_national_holiday(&old_years));
        let september_1 = Tallinn.ymd(2020, 9, 1);
        assert!(!is_national_holiday(&september_1));
    }
}
