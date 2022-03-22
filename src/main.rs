mod bar_chart;
mod nord_pool_spot;
mod price_matrix;

use chrono::{Duration, Local, TimeZone, Utc};
use chrono_tz::Europe::{Berlin, Tallinn};
use color_eyre::owo_colors::OwoColorize;

use crate::nord_pool_spot::fetch_prices_from_nord_pool;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let date_matrix = fetch_prices_from_nord_pool().await?;

    // println!("{:?}", sample_data);

    // bar_chart::draw(&date_matrix[0])?;

    let moment = nord_pool_spot::retrieve_datetime("2022-03-22", 3, &Berlin).unwrap();
    println!("{:?}", moment);

    let utc = Utc::now();
    let local = Local::now();
    let diff = local.with_timezone(&Local) - local;
    println!("{:?} | {:?} | {:?}", utc, local, diff);

    let next_day = &date_matrix[0];

    Ok(())
}
