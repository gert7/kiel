use std::{env, num::ParseIntError, str::FromStr, time::Duration};

use chrono::{Date, DateTime, TimeZone};
use chrono_tz::Tz;
use color_eyre::eyre::{self, eyre};
use rust_decimal::Decimal;
use thirtyfour::{prelude::ElementQueryable, By, DesiredCapabilities, WebDriver, WebElement};

use crate::{
    constants::MARKET_TZ,
    price_cell::PriceCell,
    price_matrix::{DateColumn, DaySlice, PriceMatrix, PricePerMwh},
};


async fn get_dates(datatable: &WebElement) -> eyre::Result<Vec<String>> {
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

async fn get_hour_from_element(e: &WebElement) -> eyre::Result<u32> {
    let e = e.text().await?;
    Ok(convert_hour_to_u32(&e)?)
}

async fn row_iteration(
    row: &WebElement,
    date_vectors: &mut PriceMatrix,
    dates: &Vec<String>,
) -> eyre::Result<()> {
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
                    Ok(dec_price) => column.cells.0.push(PriceCell {
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
                cells: DaySlice(vec![]),
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
    let driver_uri = env::var("WEBDRIVER_URI").expect("No WebDriver URI set!");
    let driver = WebDriver::new(&driver_uri, &caps).await?;

    let prices_url = env::var("DAYAHEAD_URL").expect("No dayahead site URL set!");
    driver.get(&prices_url).await?;

    let date_vectors = retrieve_prices(&driver).await;

    println!("Close await");
    driver.close().await?;
    println!("Close await done");

    Ok(date_vectors?)
}
