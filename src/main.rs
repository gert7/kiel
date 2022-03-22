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

    let _sample_data = &[
        ("00", 114u64),
        ("01", 118),
        ("02", 118),
        ("03", 119),
        ("04", 135),
        ("05", 149),
        ("06", 205),
        ("07", 253),
        ("08", 205),
        ("09", 142),
        ("10", 153),
        ("11", 159),
        ("12", 140),
        ("13", 140),
        ("14", 140),
        ("15", 160),
        ("16", 140),
        ("17", 188),
        ("18", 225),
        ("19", 235),
        ("20", 189),
        ("21", 180),
        ("22", 122),
        ("23", 167),
    ];

    // println!("{:?}", sample_data);

    // bar_chart::draw(&date_matrix[0])?;

    let moment = nord_pool_spot::retrieve_datetime("2022-03-22", 3, &Berlin).unwrap();
    println!("{:?}", moment);

    let utc = Utc::now();
    let local = Local::now();
    let diff = local.with_timezone(&Local) - local;
    println!("{:?} | {:?} | {:?}", utc, local, diff);

    Ok(())
}
