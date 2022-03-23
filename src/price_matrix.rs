use std::ops::Mul;

use chrono::{DateTime, Date};
use chrono_tz::Tz;
use rust_decimal::Decimal;

pub struct Price {
    currency_per_mwh: Decimal,
    cents_per_kwh: Decimal,
}

/// Converts currency per MWh to cents per kWh
pub fn epmh_to_cpkh(currency_per_mwh: Decimal) -> Decimal {
    currency_per_mwh / Decimal::from(10i32)
}

impl Price {
    fn new(currency_per_mwh: Decimal) -> Price {
        Price {
            currency_per_mwh,
            cents_per_kwh: epmh_to_cpkh(currency_per_mwh),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PriceCell {
    pub hour: u32,
    pub price: Decimal,
    pub moment: DateTime<Tz>,
}

#[derive(Clone, Debug)]
pub struct DateColumn {
    pub date_title: String,
    pub date: Date<Tz>,
    pub cells: Vec<PriceCell>,
}

pub type PriceMatrix = Vec<Option<DateColumn>>;