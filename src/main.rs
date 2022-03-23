mod bar_chart;
mod nord_pool_spot;
mod price_matrix;

use chrono::{Duration, Local, TimeZone, Utc};
use chrono_tz::Europe::{Berlin, Tallinn};
use color_eyre::owo_colors::OwoColorize;
use rust_decimal::Decimal;

use crate::{nord_pool_spot::fetch_prices_from_nord_pool, price_matrix::epmh_to_cpkh};

#[tokio::main]
#[doc(hidden)]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    // let date_matrix = fetch_prices_from_nord_pool().await?;

    // println!("{:?}", sample_data);

    // bar_chart::draw(&date_matrix[0])?;

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

    let original = Decimal::new(7715, 2);
    println!("{}", original);
    let converted = epmh_to_cpkh(original);
    println!("{}", converted);

    Ok(())
}
