use chrono::{Date, Duration, TimeZone};
use chrono_tz::{Tz, Europe::Berlin};
use rand::{Rng, thread_rng};
use rust_decimal::Decimal;

use crate::{price_matrix::{DaySlice, PricePerMwh}, price_cell::PriceCell};

fn random_price<R>(rng: &mut R) -> Decimal
where
    R: Rng + ?Sized,
{
    let base = (rng.gen::<u32>() % 19000) + 100; // ensure at least €1
    Decimal::new(base.into(), 2)
}

pub fn sample_day<R>(start_date: &Date<Tz>, start_hour: u32, num_hours: u32, rng: &mut R) -> DaySlice
where
    R: Rng + ?Sized,
{
    let mut vec = vec![];
    let start_dt = start_date.and_hms(start_hour, 0, 0);
    for h in 0..num_hours {
        let hour = PriceCell {
            price: PricePerMwh(random_price(rng)),
            moment: start_dt + Duration::hours(h.into()),
            tariff_price: None,
            market_hour: (h + start_hour) % 24,
        };
        vec.push(hour);
    }
    vec
}

pub fn sample_day_specified(prices: &'static [Decimal], start_hour: u32) -> DaySlice {
    let mut vec = vec![];
    let start_dt = Berlin.ymd(2022, 3, 21).and_hms(start_hour, 0, 0);
    for (i, price) in prices.iter().enumerate() {
        let offset: u32 = i.try_into().unwrap();
        let moment = start_dt + Duration::hours(offset.into());
        vec.push(PriceCell { price: PricePerMwh(*price), moment, tariff_price: None, market_hour: start_hour + offset });
    }
    vec
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
