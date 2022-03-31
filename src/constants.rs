use chrono_tz::{Europe::{Berlin, Tallinn}, Tz};
use rust_decimal::Decimal;
use lazy_static::lazy_static;

use crate::price_matrix::CentsPerKwh;

pub const MARKET_TZ: Tz = Berlin;

pub const LOCAL_TZ: Tz = Tallinn;

lazy_static! {

    pub static ref DAY_TARIFF_PRICE: CentsPerKwh = CentsPerKwh(Decimal::new(616, 2));

    pub static ref NIGHT_TARIFF_PRICE: CentsPerKwh = CentsPerKwh(Decimal::new(358, 2));

}
