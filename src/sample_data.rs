use chrono::{Duration, TimeZone, DateTime, Timelike};
use chrono_tz::{Tz, Europe::Berlin};
use color_eyre::Result;
use now::DateTimeNow;
use rand::Rng;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use color_eyre::eyre::eyre;

use crate::{price_matrix::{DaySlice, PricePerMwh}, price_cell::PriceCell, constants::PLANNING_TZ};

fn random_price<R>(rng: &mut R) -> Decimal
where
    R: Rng + ?Sized,
{
    let base = (rng.gen::<u32>() % 19000) + 100; // ensure at least €1
    Decimal::new(base.into(), 2)
}

pub fn sample_day<R>(start_date: &DateTime<Tz>, start_hour: u32, num_hours: u32, rng: &mut R) -> Result<DaySlice>
where
    R: Rng + ?Sized,
{
    let mut vec = vec![];
    let start_dt = start_date.with_hour(start_hour).ok_or(eyre!("Unable to set start_hour"))?;
    let start_dt = start_dt.beginning_of_hour();
    for h in 0..num_hours {
        let hour = PriceCell {
            price: PricePerMwh(random_price(rng)),
            moment: start_dt + Duration::hours(h.into()),
            tariff_price: None,

            market_hour: (h + start_hour) % 24,
        };
        vec.push(hour);
    }
    Ok(DaySlice(vec))
}

/// Produces sample day based on static array of decimals.
/// Tariff will be None
pub fn sample_day_specified(prices: &'static [Decimal], start_hour: u32) -> DaySlice {
    let mut vec = vec![];
    let start_dt = Berlin.ymd(2022, 3, 21).and_hms(start_hour, 0, 0);
    for (i, price) in prices.iter().enumerate() {
        let offset: u32 = i.try_into().unwrap();
        let moment = start_dt + Duration::hours(offset.into());
        vec.push(PriceCell { price: PricePerMwh(*price), moment, tariff_price: None, market_hour: start_hour + offset });
    }
    DaySlice(vec)
}

#[cfg(test)]
mod tests {
    use rand::{prelude::StdRng, SeedableRng};

    use super::*;

    #[test]
    fn random_price_in_range() {
        let mut rng = StdRng::seed_from_u64(711);
        let limit = Decimal::new(20000, 2);
        for _ in 0..10000 {
            let price = random_price(&mut rng);
            assert!(price < limit);
            assert!(price > Decimal::new(0, 0));
        }
    }
}

pub const SAMPLE_DAY_PRICES_PARTIAL: [Decimal; 8] = [
    dec!(39.43),
    dec!(134.30),
    dec!(74.10),
    dec!(190.39),
    dec!(90.39),
    dec!(190.39),
    dec!(10.39),
    dec!(33.39),
];

pub const SAMPLE_DAY_PRICES_24: [Decimal; 24] = [
    dec!(39.43),
    dec!(134.30),
    dec!(74.10),
    dec!(190.39),
    dec!(90.39),
    dec!(190.39),
    dec!(10.39),
    dec!(33.39),
    dec!(39.43),
    dec!(134.30),
    dec!(74.10),
    dec!(190.39),
    dec!(90.39),
    dec!(190.39),
    dec!(10.39),
    dec!(33.39),
    dec!(39.43),
    dec!(134.30),
    dec!(74.10),
    dec!(190.39),
    dec!(90.39),
    dec!(190.39),
    dec!(10.39),
    dec!(33.39),
];
