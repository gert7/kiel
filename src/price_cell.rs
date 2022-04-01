use crate::schema::price_cells;
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use diesel::{prelude::*, Connection, PgConnection};
use rust_decimal::Decimal;

use crate::{
    constants::{DAY_TARIFF_PRICE, MARKET_TZ, NIGHT_TARIFF_PRICE},
    price_matrix::{CentsPerKwh, PricePerMwh},
    tariff::Tariff,
};

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

    pub fn total(&self) -> PricePerMwh {
        let mut price = self.price.0;
        self.tariff_price.as_ref().map(|tariff| price += tariff.0);
        PricePerMwh(price)
    }

    pub fn get_prices_from_db<C: Connection>(connection: &C) {
        use self::price_cells::dsl::*;
        let utc = Utc::now();
        price_cells.filter(moment_utc.eq(&utc));
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

#[derive(Insertable)]
#[table_name = "price_cells"]
pub struct NewPriceCellDB<'a> {
    pub price_mwh: &'a Decimal,
    pub moment_utc: DateTime<Utc>,
    pub tariff_mwh: Option<&'a Decimal>,
    pub market_hour: i16,
}

impl<'a> NewPriceCellDB<'a> {
    fn new(pc: &'a PriceCell) -> Self {
        let tariff_mwh = pc.tariff_price.as_ref().map(|ppm| &ppm.0);
        NewPriceCellDB {
            price_mwh: &pc.price.0,
            moment_utc: pc.moment.with_timezone(&Utc),
            tariff_mwh,
            market_hour: pc.market_hour.try_into().unwrap(),
        }
    }
}
