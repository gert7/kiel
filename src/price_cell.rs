use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use color_eyre::owo_colors::OwoColorize;
use rust_decimal::Decimal;

use crate::{price_matrix::{PricePerMwh, CentsPerKwh}, tariff::Tariff, constants::{DAY_TARIFF_PRICE, NIGHT_TARIFF_PRICE, MARKET_TZ}};

#[derive(Clone, Debug, Queryable)]
pub struct PriceCell {
    pub price: PricePerMwh,
    pub moment: DateTime<Tz>,
    pub tariff_price: Option<PricePerMwh>,
    pub market_hour: u32,
}

impl PriceCell {
    pub fn get_tariff_price(
        moment: DateTime<Tz>,
        day_tariff: &CentsPerKwh,
        night_tariff: &CentsPerKwh,
    ) -> PricePerMwh {
        let tariff = Tariff::get_tariff(&moment);
        let tariff_value = match tariff {
            Tariff::Night => night_tariff,
            Tariff::Day => day_tariff,
        };
        PricePerMwh::from(tariff_value)
    }

    pub fn get_tariff_price_current(moment: DateTime<Tz>) -> PricePerMwh {
        Self::get_tariff_price(moment, &DAY_TARIFF_PRICE, &NIGHT_TARIFF_PRICE)
    }

    fn add_tariff(&mut self, day_tariff: &CentsPerKwh, night_tariff: &CentsPerKwh) {
        let tariff = Tariff::get_tariff(&self.moment);
        let tariff_value = match tariff {
            Tariff::Night => night_tariff,
            Tariff::Day => day_tariff,
        };
        self.tariff_price = Some(tariff_value.into());
    }

    fn total(&self) -> PricePerMwh {
        let mut price = self.price.0;
        self.tariff_price.as_ref().map(|tariff| price += tariff.0);
        PricePerMwh(price)
    }
}

//
// DATABASE SECTION
//

#[derive(Queryable)]
pub struct PriceCellDB {
    id: i32,
    price: Decimal,
    moment: DateTime<Utc>,
    tariff: Option<Decimal>,
    market_hour: i16,
}

impl From<PriceCellDB> for PriceCell {
    fn from(pcdb: PriceCellDB) -> Self {
        let tariff_price = pcdb.tariff.map(|d| PricePerMwh(d));
        PriceCell {
            price: PricePerMwh(pcdb.price),
            moment: pcdb.moment.with_timezone(&MARKET_TZ),
            tariff_price,
            market_hour: pcdb.market_hour.try_into().unwrap(),
        }
    }
}

use super::schema::price_cells;

#[derive(Insertable)]
#[table_name="price_cells"]
pub struct NewPriceCell<'a> {
    pub price_mwh: &'a Decimal,
    pub moment_utc: &'a DateTime<Utc>,
    pub tariff_mwh: Option<&'a Decimal>,
    pub market_hour: &'a i16,
}
