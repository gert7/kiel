use std::{num::ParseIntError, str::FromStr};

use chrono::{Date, DateTime, TimeZone};
use chrono_tz::Tz;
use color_eyre::eyre::eyre;
use rust_decimal::Decimal;
use thirtyfour::{
    prelude::{ElementQueryable, WebDriverResult},
    By, DesiredCapabilities, WebDriver, WebElement,
};

use crate::{
    constants::MarketTZ,
    price_matrix::{DateColumn, PriceCell, PriceMatrix, PricePerMwh},
};

fn convert_price_to_decimal(string: &str) -> Decimal {
    let string = string.replace(",", ".");
    Decimal::from_str(&string).unwrap() // TODO: Replace with something better
}

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

fn parse_date(date: &str, timezone: &Tz) -> Option<Date<Tz>> {
    println!("Parsing {}", date);
    let dmy = parse_date_dmy(date, timezone);
    if dmy.is_ok() {
        return Some(dmy.unwrap());
    }
    let ymd = parse_date_ymd(date, timezone);
    if ymd.is_ok() {
        return Some(ymd.unwrap());
    }
    None
}

pub fn retrieve_datetime(date: &str, hour: u32, timezone: &Tz) -> Option<DateTime<Tz>> {
    let date = parse_date(date, timezone);
    date.and_then(|d| Some(d.and_hms(hour.into(), 0, 0)))
}

async fn get_dates<'a>(datatable: &WebElement<'a>) -> color_eyre::Result<Vec<String>> {
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

async fn get_hour_from_cell<'a>(e: &WebElement<'a>) -> color_eyre::Result<u32> {
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
    let hour = get_hour_from_cell(hour_cell).await?;

    for (date_i, cell) in cells.enumerate() {
        match &mut date_vectors[date_i] {
            // check if column here isn't invalid
            Some(column) => {
                let intext = cell.text().await?;
                let dateline = &dates[date_i];
                let moment = retrieve_datetime(dateline, hour, &MarketTZ);
                match moment {
                    Some(moment) => {
                        let price_cell = PriceCell {
                            price: PricePerMwh(convert_price_to_decimal(&intext)),
                            moment: moment,
                            tariff: None,
                            market_hour: hour,
                        };
                        column.cells.push(price_cell);
                    }
                    None => eprintln!("Unable to parse dateline: {}", dateline),
                }
            }
            None => continue,
        }
        // println!("{}: {:?}", date_i, price_cell);
    }
    Ok(())
}

async fn retrieve_prices(driver: &WebDriver) -> color_eyre::Result<PriceMatrix> {
    driver.query(By::Id("datatable")).exists().await?;
    let datatable = driver.query(By::Id("datatable")).first().await?;
    let dates = get_dates(&datatable).await?;

    let datatable = datatable.query(By::Tag("tbody")).first().await?;
    let rows = datatable.find_elements(By::ClassName("data-row")).await?;

    let mut date_vectors: PriceMatrix = vec![];

    for i in 0..(dates.len()) {
        match parse_date(&dates[i], &MarketTZ) {
            Some(date) => date_vectors.push(Some(DateColumn {
                date_title: dates[i].clone(),
                date: date,
                cells: vec![],
            })),
            None => date_vectors.push(None),
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

pub async fn fetch_prices_from_nord_pool() -> color_eyre::Result<PriceMatrix> {
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

    // println!("{:?}", date_vectors[0]);

    driver.close().await?;

    Ok(date_vectors?)
}
