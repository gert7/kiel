mod bar_chart;
mod nord_pool_spot;
mod price_matrix;
mod strategy;
mod tariff;

use chrono::{Duration, Local, TimeZone, Timelike, Utc, Date};
use chrono_tz::{Europe::{Berlin, Tallinn}, Tz};
use rust_decimal::Decimal;

use crate::{
    nord_pool_spot::fetch_prices_from_nord_pool,
    price_matrix::{PriceCell, PriceCentsPerKwh, PricePerMwh},
    tariff::Tariff, strategy::PowerStrategy,
};

#[tokio::main]
#[doc(hidden)]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    // let date_matrix = fetch_prices_from_nord_pool().await?;

    // println!("{:?}", sample_data);

    // bar_chart::draw(&date_matrix[0])?;

    let tariff_day = PriceCentsPerKwh(Decimal::new(616, 2));
    let tariff_night = PriceCentsPerKwh(Decimal::new(358, 2));

    let moment = nord_pool_spot::retrieve_datetime("2022-03-22", 3, &Berlin).unwrap();
    println!("{:?}", moment);

    let local = Local::now().with_timezone(&Tallinn);
    println!("{}", local.hour());
    println!("{:?}", Tariff::get_tariff(&local));
    let local_minus = local - Duration::hours(4);
    println!("{:?}", Tariff::get_tariff(&local_minus));

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
    fn mmxxii_23_march() -> Date<Tz> { Tallinn.ymd(2022, 3, 23) }
    /// Saturday
    fn mmxxii_26_march() -> Date<Tz> { Tallinn.ymd(2022, 3, 26) }

    let date = mmxxii_23_march();
    let planned_day = strategy::DefaultStrategy::plan_day(&date, None);
    for change in planned_day {
        println!("{:?} {:?}", change.moment, change.state);
    }

    Ok(())
}
