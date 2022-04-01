#[macro_use]
extern crate diesel;
mod bar_chart;
mod config_file;
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

use chrono::{Date, Local, TimeZone, Utc, Datelike};
use chrono_tz::{
    America::Sao_Paulo,
    Europe::{Berlin, Tallinn},
};
use color_eyre::eyre;
use color_eyre::eyre::eyre;
use config_file::ConfigFile;
use constants::{MARKET_TZ, LOCAL_TZ};
use diesel::{prelude::*, expression::subselect::ValidSubselect};

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

async fn fetch_main() -> eyre::Result<()> {
    use schema::price_cells;

    let connection = database::establish_connection();

    // diesel::delete(price_cells::table).execute(&connection)?;

    let date_matrix = nord_pool_spot::fetch_prices_from_nord_pool().await?;
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

            if count == 0 {
                let tariff = price.tariff_price.as_ref().map(|o| &o.0);
                let new_price = NewPriceCellDB {
                    price_mwh: &price.price.0,
                    moment_utc: price.moment.with_timezone(&Utc),
                    tariff_mwh: tariff,
                    market_hour: price.market_hour.try_into().unwrap(),
                };

                let pcdb: PriceCellDB = diesel::insert_into(price_cells::table)
                    .values(&new_price)
                    .get_result(&connection)
                    .expect("Failed to insert price.");
            }
        }
    }

    Ok(())
}

async fn planner_main() -> eyre::Result<()> {
    let today = Utc::now().with_timezone(&LOCAL_TZ).date();
    let day_name = today.weekday();
    let config = ConfigFile::decode_config("asdf.toml")?;
    let config_today = config.get_day(&day_name);
    Ok(())
}

async fn hour_main() -> eyre::Result<()> {
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
        // fetch_main().await?;
        planner_main().await?;
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
