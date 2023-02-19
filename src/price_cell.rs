use std::ops::Range;

use crate::{
    constants::{
        DAY_TARIFF_PRICE_DECEMBER_2022, DAY_TARIFF_PRICE_OCTOBER_2022,
        NIGHT_TARIFF_PRICE_DECEMBER_2022, NIGHT_TARIFF_PRICE_OCTOBER_2022, DAY_TARIFF_PRICE_JANUARY_2023, NIGHT_TARIFF_PRICE_JANUARY_2023,
    },
    price_matrix::DaySlice,
    schema::price_cells,
};
use chrono::{Date, DateTime, TimeZone, Timelike, Utc};
use chrono_tz::Tz;
use color_eyre::eyre;
use diesel::{prelude::*, PgConnection};
use rust_decimal::Decimal;

use crate::{
    constants::{
        DAY_TARIFF_PRICE, MARKET_TZ, NIGHT_TARIFF_PRICE,
        },
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

pub fn get_hour_start_end(datetime: &DateTime<Tz>) -> Range<DateTime<Tz>> {
    let start = datetime.date().and_hms(datetime.hour(), 0, 0);
    let end = datetime.date().and_hms(datetime.hour() + 1, 0, 0);
    Range { start, end }
}

pub fn get_day_start_end(date: &Date<Tz>) -> (DateTime<Tz>, DateTime<Tz>) {
    let midnight_start = date.and_hms(0, 0, 0);
    let midnight_end = date.and_hms(23, 59, 59);
    (midnight_start, midnight_end)
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

    fn day_tariff_price<'a>(moment: &Date<Tz>) -> &'a CentsPerKwh {
        if moment < &MARKET_TZ.ymd(2022, 6, 1) {
            &DAY_TARIFF_PRICE
        } else if moment < &MARKET_TZ.ymd(2022, 12, 1) {
            &DAY_TARIFF_PRICE_OCTOBER_2022
        } else if moment < &MARKET_TZ.ymd(2023, 1, 1){
            &DAY_TARIFF_PRICE_DECEMBER_2022
        } else {
            &DAY_TARIFF_PRICE_JANUARY_2023
        }
    }

    fn night_tariff_price<'a>(moment: &Date<Tz>) -> &'a CentsPerKwh {
        if moment < &MARKET_TZ.ymd(2022, 6, 1) {
            &NIGHT_TARIFF_PRICE
        } else if moment < &MARKET_TZ.ymd(2022, 12, 1) {
            &NIGHT_TARIFF_PRICE_OCTOBER_2022
        } else if moment < &MARKET_TZ.ymd(2022, 1, 1) {
            &NIGHT_TARIFF_PRICE_DECEMBER_2022
        } else {
            &NIGHT_TARIFF_PRICE_JANUARY_2023
        }
    }

    pub fn get_tariff_price_current(moment: DateTime<Tz>) -> PricePerMwh {
        Self::get_tariff_price(
            moment,
            PriceCell::day_tariff_price(&moment.date()),
            PriceCell::night_tariff_price(&moment.date()),
        )
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

    /// Fetches all prices on the given date in its
    /// given timezone.
    pub fn get_prices_from_db(connection: &PgConnection, date: &Date<Tz>) -> DaySlice {
        use self::price_cells::dsl::*;

        let (midnight_start, midnight_end) = get_day_start_end(date);
        let cells = price_cells
            .filter(moment_utc.ge(&midnight_start))
            .filter(moment_utc.lt(&midnight_end))
            .load::<PriceCellDB>(connection)
            .expect("Unable to load price cells");
        let cells = cells.into_iter().map(|pcdb| pcdb.into()).collect();
        DaySlice(cells)
    }

    pub fn insert_cell_into_database(&self, connection: &PgConnection) -> eyre::Result<()> {
        use self::price_cells::dsl::*;

        let utc = self.moment.with_timezone(&Utc);
        let count = price_cells
            .filter(moment_utc.eq(&utc))
            .limit(5)
            .count()
            .get_result::<i64>(connection)
            .expect("Unable to count in price_cells table!");

        if count == 0 {
            let tariff = self.tariff_price.as_ref().map(|o| &o.0);
            let new_price = NewPriceCellDB {
                price_mwh: &self.price.0,
                moment_utc: self.moment.with_timezone(&Utc),
                tariff_mwh: tariff,
                market_hour: self.market_hour.try_into().unwrap(),
            };

            let pcdb: PriceCellDB = diesel::insert_into(price_cells)
                .values(&new_price)
                .get_result::<PriceCellDB>(connection)
                .expect("Failed to insert price.");
        }

        Ok(())
    }

    pub fn insert_cells_into_database(
        connection: &PgConnection,
        prices: &Vec<PriceCell>,
    ) -> eyre::Result<()> {
        for price in prices {
            price.insert_cell_into_database(connection)?;
        }
        Ok(())
    }
}

//
// DATABASE SECTION
//

#[derive(Queryable)]
pub struct PriceCellDB {
    id: i32,
    price: Decimal,
    moment_utc: DateTime<Utc>,
    tariff: Option<Decimal>,
    market_hour: i16,
    created_at: DateTime<Utc>,
}

impl From<PriceCellDB> for PriceCell {
    fn from(pcdb: PriceCellDB) -> Self {
        let tariff_price = pcdb.tariff.map(|d| PricePerMwh(d));
        PriceCell {
            price: PricePerMwh(pcdb.price),
            moment: pcdb.moment_utc.with_timezone(&MARKET_TZ),
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
