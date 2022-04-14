use std::{collections::BTreeMap, env}; 
use color_eyre::eyre;
use color_eyre::eyre::eyre;
use json::JsonValue;

use crate::{
    constants::MARKET_TZ,
    nord_pool_spot::{
        convert_hour_to_u32, convert_price_to_decimal, parse_date, retrieve_datetime,
    },
    price_cell::PriceCell,
    price_matrix::{DateColumn, PriceMatrix, PricePerMwh},
};

pub fn decode_json(body: &str) -> eyre::Result<PriceMatrix> {
    let json = json::parse(&body)?;

    let mut date_map: BTreeMap<String, Option<DateColumn>> = BTreeMap::new();

    let rows = &json["data"]["Rows"];

    if let JsonValue::Array(vec) = rows {
        for row in vec {
            let hour = row["Name"].as_str().ok_or(eyre!("Missing hour name"))?;
            let hour = convert_hour_to_u32(hour);
            let hour = match hour {
                Ok(v) => v,
                Err(_) => continue,
            };
            println!("{}", hour);
            let columns = &row["Columns"];
            if let JsonValue::Array(vec) = columns {
                for cell in vec {
                    let dateline = cell["Name"].as_str().ok_or(eyre!("Missing dateline"))?;

                    if !date_map.contains_key(dateline) {
                        let formal_date = parse_date(dateline, &MARKET_TZ)?;
                        date_map.insert(dateline.to_owned(), Some(DateColumn::new(formal_date)));
                    }

                    let price = cell["Value"].as_str().ok_or(eyre!("Missing Value"))?;
                    let moment = retrieve_datetime(dateline, hour, &MARKET_TZ)?;
                    match convert_price_to_decimal(price) {
                        Ok(dec_price) => {
                            match date_map.get_mut(dateline).unwrap() {
                                Some(cells) => cells.cells.0.push(PriceCell {
                                    price: PricePerMwh(dec_price),
                                    moment,
                                    tariff_price: Some(PriceCell::get_tariff_price_current(moment)),
                                    market_hour: hour,
                                }),
                                None => todo!(),
                            }
                        }
                        Err(_) => continue,
                    }
                }
            }
        }
    }
    let map_to_vec = date_map.into_values().collect();
    Ok(map_to_vec)
}

pub async fn fetch_json_from_nord_pool() -> eyre::Result<PriceMatrix> {
    let body = reqwest::get(env::var("JSON_URI")?).await?.text().await?;
    // let mut requestjson = File::create("out.json")?;
    // requestjson.write_all(&body.as_bytes())?;

    // let body = std::fs::read_to_string("samples/request.json")?;
    let date_vectors = decode_json(&body)?;

    println!("{:?}", date_vectors);

    Ok(date_vectors)
}
