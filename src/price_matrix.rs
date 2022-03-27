use chrono::{DateTime, Date, Duration};
use chrono_tz::Tz;
use rust_decimal::Decimal;

use crate::tariff::Tariff;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct PricePerMwh(pub Decimal);

impl PricePerMwh {
    fn new(currency_per_mwh: Decimal) -> PricePerMwh {
        PricePerMwh(currency_per_mwh)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct PriceCentsPerKwh(pub Decimal);

impl PriceCentsPerKwh {
    fn new(cents_per_kwh: Decimal) -> PriceCentsPerKwh {
        PriceCentsPerKwh(cents_per_kwh)
    }
}

impl From<&PricePerMwh> for PriceCentsPerKwh {
    fn from(e_per_mwh: &PricePerMwh) -> Self {
        PriceCentsPerKwh(e_per_mwh.0 / Decimal::from(10i32))
    }
}

impl From<&PriceCentsPerKwh> for PricePerMwh {
    fn from(c_per_kwh: &PriceCentsPerKwh) -> Self {
        PricePerMwh(c_per_kwh.0 * Decimal::from(10i32))
    }
}

#[derive(Clone, Debug)]
pub struct PriceCell {
    pub price: PricePerMwh,
    pub moment: DateTime<Tz>,
    pub tariff_price: Option<PricePerMwh>,
    pub market_hour: u32,
}

impl PriceCell {
    fn add_tariff(&mut self, day_tariff: &PriceCentsPerKwh, night_tariff: &PriceCentsPerKwh) {
        let tariff = Tariff::get_tariff(&self.moment);
        let tariff_value = match tariff {
            Tariff::Night => night_tariff,
            Tariff::Day => day_tariff,
        };
        self.tariff_price = Some(tariff_value.into());
    }
}

#[derive(Clone, Debug)]
pub struct DateColumn {
    pub date: Date<Tz>,
    pub cells: Vec<PriceCell>,
}

pub type PriceMatrix = Vec<Option<DateColumn>>;

pub type DaySlice = Vec<PriceCell>;

// #[derive(Clone, Debug)]
// pub struct DayNightSlice {
//     pub split_hour: u32,
//     pub prices: Vec<PriceCell>,
// }

pub fn add_almost_day(dt: &DateTime<Tz>) -> DateTime<Tz> {
    let hours = Duration::hours(23);
    let minutes = Duration::minutes(59);
    let seconds = Duration::seconds(59);
    *dt + hours + minutes + seconds
}

pub fn truncate_to_24_hours(day_prices: &Vec<PriceCell>) -> Vec<PriceCell> {
    let mut sorted_prices = day_prices.clone();
    sorted_prices.sort_by(|a, b| a.moment.cmp(&b.moment));
    let first = sorted_prices.first();
    match first {
        Some(init) => {
            let day_cycle = add_almost_day(&init.moment);
            let filtered_prices = sorted_prices
                .into_iter()
                .filter(|price| price.moment <= day_cycle);
            filtered_prices.collect()
        }
        None => sorted_prices,
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use crate::constants::MARKET_TZ;

    use super::*;

    #[test]
    fn converts_to_cpkh() {
        let mwh = PricePerMwh(Decimal::new(14899, 2));
        let kph = PriceCentsPerKwh::from(&mwh);
        assert!(kph.0 == Decimal::new(14899, 3));
    }

    #[test]
    fn converts_to_mh() {
        let kph = PriceCentsPerKwh(Decimal::new(948, 2));
        let mwh = PricePerMwh::from(&kph);
        assert!(mwh.0 == Decimal::new(948, 1));
    }

    #[test]
    fn adds_almost_day() {
        let date1 = MARKET_TZ.ymd(2022, 3, 3).and_hms(0, 0, 0);
        let added = add_almost_day(&date1);
        assert!(added == MARKET_TZ.ymd(2022, 3, 3).and_hms(23, 59, 59));
    }
}
