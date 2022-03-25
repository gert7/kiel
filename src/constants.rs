use chrono_tz::{Europe::{Berlin, Tallinn}, Tz};
use rust_decimal::Decimal;
use lazy_static::lazy_static;

use crate::price_matrix::PriceCentsPerKwh;

pub const MarketTZ: Tz = Berlin;

pub const LocalTZ: Tz = Tallinn;

lazy_static! {

    pub static ref DayTariffPrice: PriceCentsPerKwh = PriceCentsPerKwh(Decimal::new(616, 2));

    pub static ref NightTariffPrice: PriceCentsPerKwh = PriceCentsPerKwh(Decimal::new(358, 2));

}
