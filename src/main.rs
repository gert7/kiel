// fn main() -> reqwest::Result<()>{
//     let ql_json = std::fs::read_to_string("query.json").unwrap();

//     println!("Hello, world!");
//     let body = reqwest::blocking::get("https://www.elektrikell.ee/")?.text()?;
//     println!("{:?}", body);
//     let client = reqwest::blocking::Client::new();
//     let res = client.post("https://krxjhu765fba7iqriz2xaivbxa.appsync-api.eu-west-1.amazonaws.com/graphql")
//         .body(ql_json)
//         .send()?;
//     let res_text = res.text()?;
//     print!("{:?}", res_text);
//     Ok(())
// }

use std::{str::FromStr, time::Duration};

use color_eyre::{owo_colors::OwoColorize, Result};
use rust_decimal::Decimal;
use thirtyfour::{prelude::ElementQueryable, By, DesiredCapabilities, WebDriver, WebElement};

fn convert_to_decimal(string: &str) -> Decimal {
    let string = string.replace(",", ".");
    Decimal::from_str(&string).unwrap() // TODO: Replace with something better
}

fn convert_hour_to_u32(string: &str) -> Option<u32> {
    let mut string2 = string.to_owned();
    string2.truncate(2);

    string2.parse::<u32>().ok()
}

#[derive(Clone, Debug)]
struct PriceCell {
    hour: u32,
    price: Decimal,
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

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

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

    let mut date_vectors: Vec<Vec<PriceCell>> = vec![vec![]; dates.len()];

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
            println!("{}: {:?}", date_i, price_cell);
            date_vectors[date_i].push(price_cell);
        }
    }

    driver.close().await?;

    Ok(())
}
