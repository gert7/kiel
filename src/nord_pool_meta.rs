use std::{num::ParseIntError, str::FromStr};

use chrono::{Date, DateTime, TimeZone};
use chrono_tz::Tz;
use color_eyre::eyre;
use color_eyre::eyre::eyre;
use rust_decimal::Decimal;


pub fn convert_price_to_decimal(string: &str) -> eyre::Result<Decimal> {
    let string = string.replace(",", ".");
    Ok(Decimal::from_str(&string)?)
}

/// Converts an hour string taken directly from an element,
/// such as `01 - 02` to a `u32`.
pub fn convert_hour_to_u32(string: &str) -> Result<u32, ParseIntError> {
    let mut string2 = string.to_owned();
    string2.truncate(2);

    println!("{string}");
    Ok(string2.parse::<u32>()?)
}

fn parse_date_dmy(date: &str, timezone: &Tz) -> Result<Date<Tz>, ParseIntError> {
    // DD-MM-YYYY
    let day: u32 = date[0..=1].parse()?;
    let month: u32 = date[3..=4].parse()?;
    let year: i32 = date[6..=9].parse()?;
    Ok(timezone.ymd(year, month, day))
}

fn parse_date_ymd(date: &str, timezone: &Tz) -> Result<Date<Tz>, ParseIntError> {
    // YYYY-MM-DD
    let year: i32 = date[0..=3].parse()?;
    let month: u32 = date[5..=6].parse()?;
    let day: u32 = date[8..=9].parse()?;
    Ok(timezone.ymd(year, month, day))
}

pub fn parse_date(date: &str, timezone: &Tz) -> eyre::Result<Date<Tz>> {
    let dmy = parse_date_dmy(date, timezone);
    if dmy.is_ok() {
        return Ok(dmy.unwrap());
    }
    let ymd = parse_date_ymd(date, timezone);
    if ymd.is_ok() {
        return Ok(ymd.unwrap());
    }
    Err(eyre!("Unable to parse dateline: {}", date))
}

pub fn retrieve_datetime(date: &str, hour: u32, timezone: &Tz) -> eyre::Result<DateTime<Tz>> {
    let date = parse_date(date, timezone)?;
    date.and_hms_opt(hour, 0, 0)
        .ok_or(eyre!("Invalid hour: {} {}", date, hour))
}