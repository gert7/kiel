mod bar_chart;
mod constants;
mod nord_pool_spot;
mod price_matrix;
mod sample_data;
mod strategy;
mod tariff;

use chrono::{Date, Duration, Local, TimeZone, Timelike, Utc};
use chrono_tz::{
    America::Argentina::Buenos_Aires,
    Europe::{Berlin, Tallinn},
    Tz,
};
use rand::thread_rng;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::{
    constants::MARKET_TZ,
    price_matrix::{add_almost_day, PriceCentsPerKwh},
    sample_data::sample_day_specified,
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

#[tokio::main]
#[doc(hidden)]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    // let date_matrix = fetch_prices_from_nord_pool().await?;

    // for date in date_matrix {
    //     println!("{:?}", date);
    // }

    // BAR CHART SECTION
    // bar_chart::draw(&date_matrix[0])?;

    let tariff_day = PriceCentsPerKwh(Decimal::new(616, 2));
    let tariff_night = PriceCentsPerKwh(Decimal::new(358, 2));

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

    // let date = mmxxii_23_march();
    // let planned_day = strategy::DefaultStrategy::plan_day(None);
    // for change in planned_day {
    //     println!("{:?} {:?}", change.moment, change.state);
    // }

    let date1 = MARKET_TZ.ymd(2022, 3, 3).and_hms(0, 0, 0);
    let date2 = add_almost_day(&date1);
    println!("{} {}", date1, date2);

    let date = MARKET_TZ.ymd(2022, 3, 3);

    // bar_chart::draw(&sd)?;

    let sam = sample_day_specified(&SAMPLE_DAY_PRICES, 14);

    for d in sam {
        println!("{:?}", d);
    }

    Ok(())
}
