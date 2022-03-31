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
mod database;

use std::time::Duration;

use nord_pool_spot::fetch_prices_from_nord_pool;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    tokio::time::sleep(Duration::from_secs(2)).await;
    println!("Price fetch!");
    let date_matrix = fetch_prices_from_nord_pool().await?;
    println!("{:?}", date_matrix);
    Ok(())
}
