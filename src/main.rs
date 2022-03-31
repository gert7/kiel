#[macro_use]
extern crate diesel;
mod bar_chart;
mod constants;
mod models;
mod nord_pool_spot;
mod price_cell;
mod price_matrix;
mod sample_data;
mod schema;
mod strategy;
mod tariff;

use std::env;

use chrono::{Date, Local, TimeZone, Utc};
use chrono_tz::{
    Europe::{Berlin, Tallinn},
    Tz,
};
use diesel::prelude::*;
use diesel::{Connection, PgConnection};
use dotenv::dotenv;

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::{
    price_cell::{NewPriceCell, PriceCell, PriceCellDB},
    price_matrix::CentsPerKwh,
};

const SAMPLE_DAY_PRICES: [Decimal; 8] = [
    dec!(39.43),
    dec!(134.30),
    dec!(74.10),
    dec!(190.39),
    dec!(90.39),
    dec!(190.39),
    dec!(10.39),
    dec!(33.39),
];

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("No DATABASE_URL set!");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[tokio::main]
#[doc(hidden)]
async fn main() -> color_eyre::Result<()> {
    use schema::price_cells;

    color_eyre::install()?;

    let connection = establish_connection();

    let results = price_cells::table
        .filter(price_cells::market_hour.eq(19))
        .limit(5)
        .load::<PriceCellDB>(&connection)
        .expect("Erroir");

    let new_price = NewPriceCell {
        price_mwh: &dec!(121.94),
        moment_utc: &Tallinn
            .ymd(2022, 3, 19)
            .and_hms(12, 43, 12)
            .with_timezone(&Utc),
        tariff_mwh: None,
        market_hour: &12,
    };

    let a: PriceCellDB = diesel::insert_into(price_cells::table)
        .values(&new_price)
        .get_result(&connection)
        .expect("Failed to insert");

    // let date_matrix = fetch_prices_from_nord_pool().await?;

    // BAR CHART SECTION
    // bar_chart::draw(&date_matrix[0])?;

    let tariff_day = CentsPerKwh(Decimal::new(616, 2));
    let tariff_night = CentsPerKwh(Decimal::new(358, 2));

    let moment = nord_pool_spot::retrieve_datetime("2022-03-22", 3, &Berlin).unwrap();
    println!("{:?}", moment);

    let local = Local::now().with_timezone(&Tallinn);

    // let next_day = &date_matrix[0];

    // println!("{:?}", date_matrix);

    // match next_day {
    //     Some(v) => {
    //         for cell in &v.cells {
    //             let difference = cell.moment - local;
    //             println!("Time until: {:?}", difference.num_minutes() / 60);
    //         }
    //     },
    //     None => todo!(),
    // }

    // let kph = PriceCentsPerKwh(Decimal::new(948, 2));
    // let mwh = PriceCentsPerKwh::from(&kph);
    // println!("{} {}", kph, mwh);

    /// Wednesday
    fn mmxxii_23_march() -> Date<Tz> {
        Berlin.ymd(2022, 3, 23)
    }
    /// Saturday
    fn mmxxii_26_march() -> Date<Tz> {
        Berlin.ymd(2022, 3, 26)
    }

    Ok(())
}
