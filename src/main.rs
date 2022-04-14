#[macro_use]
extern crate diesel;
mod apply;
mod bar_chart;
mod config_file;
mod constants;
mod convars;
mod database;
mod integration_test;
mod nord_pool_spot;
mod nord_pool_spot_json;
mod overrides;
mod price_cell;
mod price_matrix;
mod proc_mutex;
mod sample_data;
mod schema;
mod strategy;
mod switch_records;
mod tariff;

use std::{env, fs::File, io::Write, time::Duration};

use chrono::{Date, DateTime, Datelike, Local, TimeZone, Utc};
use chrono_tz::{
    America::Sao_Paulo,
    Europe::{Berlin, Tallinn},
    Tz,
};
use color_eyre::eyre;
use color_eyre::eyre::eyre;
use config_file::ConfigFile;
use constants::{DEFAULT_CONFIG_FILENAME, LOCAL_TZ, MARKET_TZ, PLANNING_TZ};
use diesel::prelude::*;

use price_cell::get_hour_start_end;
use proc_mutex::wait_for_file;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use strategy::{power_state_model::PowerStateDB, PowerState, PriceChangeUnit};

use crate::{
    apply::apply_power_state,
    config_file::DayBasePlan,
    price_cell::{NewPriceCellDB, PriceCell, PriceCellDB},
    price_matrix::CentsPerKwh,
    strategy::default::TariffStrategy,
};

async fn fetch_main() -> eyre::Result<()> {
    let connection = database::establish_connection();

    let date_matrix = nord_pool_spot_json::fetch_json_from_nord_pool().await?;
    price_matrix::insert_matrix_to_database(&connection, &date_matrix)?;

    Ok(())
}

fn get_power_state(datetime: &DateTime<Tz>, states: &Vec<PriceChangeUnit>) -> Option<PowerState> {
    let mut candidate: Option<PowerState> = None;
    for pcu in states {
        if pcu.moment <= *datetime {
            candidate = Some(pcu.state);
        } else if pcu.moment > *datetime {
            break;
        }
    }
    candidate
}

fn get_power_state_exact(
    datetime: &DateTime<Tz>,
    states: &Vec<PriceChangeUnit>,
) -> Option<PowerState> {
    let range = get_hour_start_end(datetime);
    for pcu in states {
        if range.contains(&pcu.moment) {
            println!("Range found: {:?}: {:?}", pcu.moment, range);
            return Some(pcu.state);
        } else {
            // println!("Range doesn't contain {:?}: {:?}", pcu.moment, range);
        }
    }
    None
}

async fn planner_main() -> eyre::Result<()> {
    let connection = database::establish_connection();
    // let (cfdb, config) = ConfigFile::fetch_with_default(&connection, DEFAULT_CONFIG_FILENAME)?;
    // let conf_id = match cfdb {
    //     Some(cfdb) => Some(cfdb.id),
    //     None => None,
    // };
    let (conf_id, config) =
        ConfigFile::fetch_with_default_inserting(&connection, DEFAULT_CONFIG_FILENAME)?;
    println!("conf id {:?}", conf_id);

    let now = Utc::now().with_timezone(&PLANNING_TZ);
    let today = now.date();

    let cached_states = PowerStateDB::get_day_from_database(&connection, &today, Some(conf_id))?;
    let exact_known_state = get_power_state_exact(&now, &cached_states);
    println!("Current cached state: {:?}", exact_known_state);

    match exact_known_state {
        Some(known_state) => apply_power_state(&connection, &known_state).await?,
        None => {
            let config_today = config.get_day(&today.weekday());

            let pdb = PriceCell::get_prices_from_db(&connection, &today);

            let base = config_today
                .base
                .unwrap_or(DayBasePlan::Tariff(TariffStrategy));
            let base_prices = base.get_hour_strategy().plan_day_full(&pdb, &today);

            let mut strategy_result = match config_today.strategy {
                Some(strategy) => strategy.get_day_strategy().plan_day_masked(&base_prices),
                None => base_prices,
            };

            overrides::apply_overrides(&mut strategy_result, &config, &LOCAL_TZ);

            PowerStateDB::insert_day_into_database(&connection, &strategy_result, Some(conf_id));
            for pcu in &strategy_result {
                println!("{:?}", pcu);
            }

            let state = get_power_state_exact(&now, &strategy_result);
            if let Some(state) = state {
                apply_power_state(&connection, &state).await?;
            }
        }
    }

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
    dotenv::from_path("/etc/kiel.d/.env")?;
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
            "".to_owned()
        }
    };

    if second == "--fetch" {
        fetch_main().await?;
    } else if second == "--hour" {
        // hour_main().await?;
        planner_main().await?;
    } else {
        // let a = nord_pool_spot_json::fetch_json_from_nord_pool().await?;
        eprintln!("Unknown mode: {}", second);
    }

    lockfile
        .write(b"rub a dub dub thanks for the grub")
        .unwrap();

    Ok(())
}
