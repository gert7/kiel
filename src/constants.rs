use std::ops::Range;

use chrono_tz::{Europe::{Berlin, Tallinn}, Tz};
use lazy_static::lazy_static;
use rust_decimal_macros::dec;

use crate::price_matrix::CentsPerKwh;

pub const DEFAULT_CONFIG_FILENAME: &str = "/etc/kiel.d/default.toml";

pub const MARKET_TZ: Tz = Berlin;

pub const LOCAL_TZ: Tz = Tallinn;

pub const PLANNING_TZ: Tz = MARKET_TZ;

pub const HOURS_OF_DAY: Range<u8> = Range { start: 0, end: 24 };

lazy_static! {

    pub static ref DAY_TARIFF_PRICE: CentsPerKwh = CentsPerKwh(dec!(6.65));

    pub static ref NIGHT_TARIFF_PRICE: CentsPerKwh = CentsPerKwh(dec!(3.86));

}

pub const CVAR_CONFIG_FAILURE_COUNT: &str = "config_failures";