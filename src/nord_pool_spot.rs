use std::{num::ParseIntError, str::FromStr, time::Duration};

use chrono::{Date, DateTime, TimeZone};
use chrono_tz::Tz;
use color_eyre::{
    eyre::{self, eyre},
    owo_colors::OwoColorize,
};
use rust_decimal::Decimal;
use thirtyfour::{
    prelude::{ElementQueryable, WebDriverResult},
    By, DesiredCapabilities, WebDriver, WebElement,
};

use crate::{
    constants::{DAY_TARIFF_PRICE, MARKET_TZ, NIGHT_TARIFF_PRICE},
    price_matrix::{DateColumn, PriceCell, PriceMatrix, PricePerMwh},
};

fn convert_price_to_decimal(string: &str) -> eyre::Result<Decimal> {
    let string = string.replace(",", ".");
    Ok(Decimal::from_str(&string)?)
}

/// Converts an hour string taken directly from an element,
/// such as `01 - 02` to a `u32`.
fn convert_hour_to_u32(string: &str) -> Result<u32, ParseIntError> {
    let mut string2 = string.to_owned();
    string2.truncate(2);

    Ok(string2.parse::<u32>()?)
}

fn parse_date_dmy(date: &str, timezone: &Tz) -> Result<Date<Tz>, ParseIntError> {
    // DD-MM-YYYY
    let day: u32 = date[0..=1].parse()?;
    let month: u32 = date[3..=4].parse()?;
    let year: i32 = date[6..=9].parse()?;
    Ok(timezone.ymd(year, month, day))
}

fn parse_date_ymd(date: &str, timezone: &Tz) -> Result<Date<Tz>, ParseIntError> {
    // YYYY-MM-DD
    let year: i32 = date[0..=3].parse()?;
    let month: u32 = date[5..=6].parse()?;
    let day: u32 = date[8..=9].parse()?;
    Ok(timezone.ymd(year, month, day))
}

fn parse_date(date: &str, timezone: &Tz) -> eyre::Result<Date<Tz>> {
    let dmy = parse_date_dmy(date, timezone);
    if dmy.is_ok() {
        return Ok(dmy.unwrap());
    }
    let ymd = parse_date_ymd(date, timezone);
    if ymd.is_ok() {
        return Ok(ymd.unwrap());
    }
    Err(eyre!("Unable to parse dateline: {}", date))
}

pub fn retrieve_datetime(date: &str, hour: u32, timezone: &Tz) -> eyre::Result<DateTime<Tz>> {
    let date = parse_date(date, timezone)?;
    date.and_hms_opt(hour, 0, 0)
        .ok_or(eyre!("Invalid hour: {} {}", date, hour))
}

async fn get_dates<'a>(datatable: &WebElement<'a>) -> eyre::Result<Vec<String>> {
    let mut vec = vec![];
    let headers = datatable
        .query(By::ClassName("column-headers"))
        .first()
        .await?;
    let headers = headers.find_elements(By::Tag("th")).await?;
    for header in headers {
        let text = header.text().await?;
        let first_alphanumeric = text.as_bytes().iter().find(|&e| !e.is_ascii_whitespace());
        if first_alphanumeric.is_some() {
            vec.push(text);
        }
    }
    println!("{:?}", vec);
    Ok(vec)
}

async fn get_hour_from_element<'a>(e: &WebElement<'a>) -> eyre::Result<u32> {
    let e = e.text().await?;
    Ok(convert_hour_to_u32(&e)?)
}

async fn row_iteration<'a>(
    row: &WebElement<'a>,
    date_vectors: &mut PriceMatrix,
    dates: &Vec<String>,
) -> color_eyre::Result<()> {
    let cells = row.find_elements(By::Tag("td")).await?;
    let mut cells = cells.iter();
    let hour_cell = match cells.next() {
        Some(a) => a,
        None => return Err(eyre!("Missing hour cell?")),
    };
    let hour = get_hour_from_element(hour_cell).await?;

    for (date_i, cell) in cells.enumerate() {
        match &mut date_vectors[date_i] {
            // check if column here isn't invalid
            Some(column) => {
                let intext = cell.text().await?;
                let dateline = &dates[date_i];
                let moment = retrieve_datetime(dateline, hour, &MARKET_TZ)?;
                match convert_price_to_decimal(&intext) {
                    Ok(dec_price) => column.cells.push(PriceCell {
                        price: PricePerMwh(dec_price),
                        moment: moment,
                        tariff_price: Some(PriceCell::get_tariff_price_current(moment)),
                        market_hour: hour,
                    }),
                    Err(_) => continue,
                }
            }
            None => continue,
        }
        // println!("{}: {:?}", date_i, price_cell);
    }
    Ok(())
}

async fn retrieve_prices(driver: &WebDriver) -> eyre::Result<PriceMatrix> {
    driver.query(By::Id("datatable")).exists().await?;
    let datatable = driver.query(By::Id("datatable")).first().await?;
    let dates = get_dates(&datatable).await?;

    let datatable = datatable.query(By::Tag("tbody")).first().await?;
    let rows = datatable.find_elements(By::ClassName("data-row")).await?;

    let mut date_vectors: PriceMatrix = vec![];

    for i in 0..(dates.len()) {
        match parse_date(&dates[i], &MARKET_TZ) {
            Ok(date) => date_vectors.push(Some(DateColumn {
                date,
                cells: vec![],
            })),
            Err(_) => date_vectors.push(None),
        }
    }

    for row in rows.iter() {
        match row_iteration(&row, &mut date_vectors, &dates).await {
            Ok(_) => (),
            Err(e) => eprintln!("{}", e),
        }
    }

    Ok(date_vectors)
}

pub async fn fetch_prices_from_nord_pool() -> eyre::Result<PriceMatrix> {
    let mut caps = DesiredCapabilities::chrome();
    caps.add_chrome_arg("--enable-automation")?;
    let driver = WebDriver::new("http://localhost:4444/wd/hub", &caps).await?;

    driver
        .get(
            "https://www.nordpoolgroup.com/Market-data1/Dayahead/Area-Prices/EE/Hourly/?view=table",
        )
        .await?;

    // tokio::time::sleep(Duration::from_secs(5)).await;

    let date_vectors = retrieve_prices(&driver).await;

    println!("Close await");
    driver.close().await?;
    println!("Close await done");

    Ok(date_vectors?)
}
