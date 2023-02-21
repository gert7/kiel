#[macro_use]
extern crate diesel;
mod apply;
// mod bar_chart;
mod config_file;
mod constants;
mod convars;
mod database;
mod holidays;
mod integration_test;
// mod nord_pool_spot;
mod nord_pool_meta;
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

use std::{io::Write, ops::Add};

use chrono::{DateTime, Datelike, Utc};
use chrono_tz::Tz;
use color_eyre::eyre;
use config_file::ConfigFile;
use constants::{DEFAULT_CONFIG_FILENAME, LOCAL_TZ, PLANNING_TZ};

use price_cell::get_hour_start_end;
use proc_mutex::wait_for_file;
use strategy::{power_state_model::PowerStateDB, PowerState, PriceChangeUnit};

use crate::{
    apply::apply_power_state, config_file::DayBasePlan, price_cell::PriceCell,
    strategy::default::TariffStrategy,
};

async fn fetch_main() -> eyre::Result<()> {
    let connection = database::establish_connection();

    let date_matrix = nord_pool_spot_json::fetch_json_from_nord_pool().await?;
    price_matrix::insert_matrix_to_database(&connection, &date_matrix)?;

    Ok(())
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

fn planner_main<'a>(force_recalculate: bool, moment: DateTime<Tz>) -> eyre::Result<()> {
    let connection = database::establish_connection();
    let (conf_id, config) =
        ConfigFile::fetch_with_default_inserting(&connection, DEFAULT_CONFIG_FILENAME)?;
    println!("conf id {:?}", conf_id);

    let date = moment.date();

    let cached_states = PowerStateDB::get_day_from_database(&connection, &date, Some(conf_id))?;
    let exact_known_state = get_power_state_exact(&moment, &cached_states);
    println!("Current cached state: {:?}", exact_known_state);

    if let Some(known_state) = exact_known_state {
        if !force_recalculate {
            // apply_power_state(&connection, &known_state).await?;
            return Ok(());
        }
    }

    let config_day = config.get_day(&date.weekday());
    println!("{:?}", config_day);

    let pdb = PriceCell::get_prices_from_db(&connection, &date);

    let base = config_day
        .base
        .unwrap_or(DayBasePlan::Tariff(TariffStrategy));
    let base_prices = base.get_hour_strategy().plan_day_full(&pdb, &date);

    let mut strategy_result = match config_day.strategy {
        Some(strategy) => strategy.get_day_strategy().plan_day_masked(&base_prices),
        None => base_prices,
    };

    overrides::apply_overrides(&mut strategy_result, &config, &LOCAL_TZ);

    PowerStateDB::insert_day_into_database(&connection, &strategy_result, Some(conf_id));
    for pcu in &strategy_result {
        println!("{:?}", pcu);
    }

    Ok(())
}

async fn enact_now(now: DateTime<Tz>) -> eyre::Result<()> {
    let connection = database::establish_connection();
    let (conf_id, _) =
        ConfigFile::fetch_with_default_inserting(&connection, DEFAULT_CONFIG_FILENAME)?;
    let date = now.date();
    let cached_states = PowerStateDB::get_day_from_database(&connection, &date, Some(conf_id))?;
    let exact_known_state = get_power_state_exact(&now, &cached_states);
    if let Some(known_state) = exact_known_state {
        apply_power_state(&connection, &known_state).await?;
    }
    Ok(())
}

#[tokio::main]
// #[doc(hidden)]
async fn main() -> color_eyre::Result<()> {
    dotenv::from_path("/etc/kiel.d/.env")?;
    color_eyre::install()?;

    println!("[LF] getting");
    let mut lockfile = wait_for_file();
    println!("[LF] file got");

    let mut args = std::env::args();
    let second = args.nth(1);
    let second = match second {
        Some(v) => v,
        None => {
            eprintln!("\nPlease specify an execution mode:");
            eprintln!("  --fetch");
            eprintln!("  --hour");
            eprintln!("  --hour-force");
            eprintln!("  --reinsert-config [FILENAME]");
            "".to_owned()
        }
    };

    let now = Utc::now().with_timezone(&PLANNING_TZ);
    // let today = now.date_naive();
    // println!("today {}", today);
    let tomorrow = now.add(chrono::Duration::days(1));
    println!("tomorrow {}", tomorrow);

    let mut force_recalculate = false;

    if second == "--fetch" {
        fetch_main().await?;
        force_recalculate = true;
    } else if second == "--hour" {
        // Does nothing special in the condition itself.
        // Do not delete
        //
        // hour_main().await?;
    } else if second == "--hour-force" {
        println!("Forcing recalculation...");
        force_recalculate = true;
    } else if second == "--reinsert-config" {
        let third = std::env::args().nth(2);
        let filename = third.unwrap_or("default.toml".to_owned());
        println!("Reinserting crate-local configuration: {}", filename);
        let connection = database::establish_connection();
        let default_toml = std::fs::read_to_string(filename)?;
        ConfigFile::insert_string(&connection, &default_toml)?;
        force_recalculate = true;
    } else {
        // let a = nord_pool_spot_json::fetch_json_from_nord_pool().await?;
        eprintln!("Unknown mode: {}", second);
    }

    planner_main(force_recalculate, now)?;
    planner_main(force_recalculate, tomorrow)?;
    enact_now(now).await?;

    lockfile
        .write(b"rub a dub dub thanks for the grub")?;

    Ok(())
}
