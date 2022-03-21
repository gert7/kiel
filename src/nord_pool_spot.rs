use std::str::FromStr;

use rust_decimal::Decimal;
use thirtyfour::{WebElement, By, prelude::ElementQueryable, DesiredCapabilities, WebDriver};

use crate::price_matrix::{PriceMatrix, DateColumn, PriceCell};


fn convert_to_decimal(string: &str) -> Decimal {
    let string = string.replace(",", ".");
    Decimal::from_str(&string).unwrap() // TODO: Replace with something better
}

fn convert_hour_to_u32(string: &str) -> Option<u32> {
    let mut string2 = string.to_owned();
    string2.truncate(2);

    string2.parse::<u32>().ok()
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

    driver.query(By::Id("datatable")).exists().await?;
    let datatable = driver.query(By::Id("datatable")).first().await?;
    let dates = get_dates(&datatable).await?;

    let datatable = datatable.query(By::Tag("tbody")).first().await?;
    let rows = datatable.find_elements(By::ClassName("data-row")).await?;

    let mut date_vectors: PriceMatrix = vec![];

    for i in 0..(dates.len()) {
        date_vectors.push(DateColumn {
            date: dates[i].clone(),
            cells: vec![],
        })
    }

    for row in rows.iter() {
        let cells = row.find_elements(By::Tag("td")).await?;
        let mut cells = cells.iter();
        let hour = match cells.next() {
            Some(e) => {
                let e = e.text().await?;
                convert_hour_to_u32(&e)
            }
            None => continue,
        };

        let hour = match hour {
            Some(h) => h,
            None => continue,
        };

        for (date_i, cell) in cells.enumerate() {
            let intext = cell.text().await?;
            let price_cell = PriceCell {
                hour: hour,
                price: convert_to_decimal(&intext),
            };
            // println!("{}: {:?}", date_i, price_cell);
            date_vectors[date_i].cells.push(price_cell);
        }
    }

    // println!("{:?}", date_vectors[0]);

    driver.close().await?;

    Ok(date_vectors)
}
