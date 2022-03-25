use chrono::{DateTime, Date};
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
    pub tariff: Option<PricePerMwh>,
    pub market_hour: u32,
}

impl PriceCell {
    fn add_tariff(&mut self, day_tariff: &PriceCentsPerKwh, night_tariff: &PriceCentsPerKwh) {
        let tariff = Tariff::get_tariff(&self.moment);
        let tariff_value = match tariff {
            Tariff::Night => night_tariff,
            Tariff::Day => day_tariff,
        };
        self.tariff = Some(tariff_value.into());
    }
}

#[derive(Clone, Debug)]
pub struct DateColumn {
    pub date_title: String,
    pub date: Date<Tz>,
    pub cells: Vec<PriceCell>,
}

pub type PriceMatrix = Vec<Option<DateColumn>>;

#[cfg(test)]
mod tests {
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
}
