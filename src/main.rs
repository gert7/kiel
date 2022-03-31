#[macro_use]
extern crate diesel;
mod bar_chart;
mod constants;
mod database;
mod nord_pool_spot;
mod price_cell;
mod price_matrix;
mod proc_mutex;
mod sample_data;
mod schema;
mod strategy;
mod tariff;

use std::{env, fs::File, io::Write, time::Duration};

use chrono::{Date, Local, TimeZone, Utc};
use chrono_tz::{
    America::Sao_Paulo,
    Europe::{Berlin, Tallinn},
    Tz,
};
use color_eyre::owo_colors::OwoColorize;
use diesel::prelude::*;
use diesel::{Connection, PgConnection};
use dotenv::dotenv;

use proc_mutex::wait_for_file;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::{
    price_cell::{NewPriceCellDB, PriceCell, PriceCellDB},
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

async fn fetch_main() -> color_eyre::Result<()> {
    use schema::price_cells;

    let connection = database::establish_connection();
    let date_matrix = nord_pool_spot::fetch_prices_from_nord_pool().await?;
    println!("{:?}", date_matrix);
    let date_matrix = date_matrix
        .iter()
        .filter(|o| o.is_some())
        .map(|o| o.as_ref().unwrap());

    for date in date_matrix {
        for price in &date.cells {
            let utc = price.moment.with_timezone(&Utc);
            let count = price_cells::table
                .filter(price_cells::moment_utc.eq(&utc))
                .limit(5)
                .count()
                .get_result::<i64>(&connection)
                .expect("Unable to count in price_cells table!");
            println!("{}", count);
        }
    }
    Ok(())
}

async fn hour_main() -> color_eyre::Result<()> {
    use schema::price_cells;
    let connection = database::establish_connection();

    let results = price_cells::table
        .filter(price_cells::market_hour.eq(19))
        .limit(5)
        .load::<PriceCellDB>(&connection)
        .expect("Erroir");

    let new_price = NewPriceCellDB {
        price_mwh: &dec!(121.94),
        moment_utc: Sao_Paulo
            .ymd(2022, 3, 19)
            .and_hms(12, 43, 12)
            .with_timezone(&Utc),
        tariff_mwh: None,
        market_hour: 12,
    };

    let a: PriceCellDB = diesel::insert_into(price_cells::table)
        .values(&new_price)
        .get_result(&connection)
        .expect("Failed to insert");

    let b: PriceCell = a.into();

    // BAR CHART SECTION
    // bar_chart::draw(&date_matrix[0])?;

    let tariff_day = CentsPerKwh(Decimal::new(616, 2));
    let tariff_night = CentsPerKwh(Decimal::new(358, 2));

    let moment = nord_pool_spot::retrieve_datetime("2022-03-22", 3, &Berlin).unwrap();
    println!("{:?}", moment);

    let local = Local::now().with_timezone(&Tallinn);

    Ok(())
}

#[tokio::main]
#[doc(hidden)]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut lockfile = wait_for_file();

    let mut args = std::env::args();
    let second = args.nth(1);
    let second = match second {
        Some(v) => v,
        None => {
            eprintln!("\nPlease specify an execution mode:");
            eprintln!("  --fetch");
            eprintln!("  --hour\n");
            std::process::exit(1);
        }
    };

    if second == "--fetch" {
        fetch_main().await?;
    } else if second == "--hour" {
        hour_main().await?;
    } else {
        eprintln!("Unknown mode: {}", second);
    }

    lockfile
        .write(b"rub a dub dub thanks for the grub")
        .unwrap();

    Ok(())
}
