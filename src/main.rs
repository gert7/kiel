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

mod price_matrix;
mod nord_pool_spot;

use crossterm::{event::DisableMouseCapture, execute, terminal::EnterAlternateScreen};
use rust_decimal::{prelude::ToPrimitive};
use tui::{
    backend::CrosstermBackend,
    style::{Color, Style},
    widgets::{BarChart, Block, Borders},
    Terminal,
};

use crate::{price_matrix::{DateColumn, PriceCell}, nord_pool_spot::fetch_prices_from_nord_pool};

fn price_cell_vec_to_chart_data(dc: &DateColumn) -> Vec<(String, u64)> {
    let mut result = vec![];
    for cell in &dc.cells {
        let label = format!("{:02}", cell.hour);
        let value = cell.price.to_u64().unwrap();
        result.push((label, value));
    }
    result
}

fn chart_data_as_str_ref<'a>(invec: &'a Vec<(String, u64)>) -> Vec<(&'a str, u64)> {
    let mut result = vec![];
    for item in invec {
        result.push((item.0.as_ref(), item.1));
    }
    result
}

fn date_chart_max(cells: &[(&str, u64)]) -> u64 {
    let prices = cells.iter().map(|c| c.1);
    prices.max().unwrap()
}

fn price_cell_max(cells: &Vec<PriceCell>) -> u64 {
    let prices = cells.iter().map(|c| c.price.to_u64().unwrap());
    prices.max().unwrap()
}

fn draw(bar_chart_data: &[(&str, u64)], bar_chart_max: u64, title: &str) -> color_eyre::Result<()> {
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, DisableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let mut size = f.size();
        size.height = 20;

        let chart = BarChart::default()
            .block(Block::default().title(title).borders(Borders::ALL))
            .bar_width(4)
            .bar_gap(1)
            .bar_style(Style::default().fg(Color::Yellow))
            .value_style(Style::default().fg(Color::Black).bg(Color::Yellow))
            .label_style(Style::default().fg(Color::White))
            .data(bar_chart_data)
            .max(bar_chart_max);
        f.render_widget(chart, size);
    })?;

    Ok(())
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let date_matrix = fetch_prices_from_nord_pool().await?;

    let sample_data = &[
        ("00", 114u64),
        ("01", 118),
        ("02", 118),
        ("03", 119),
        ("04", 135),
        ("05", 149),
        ("06", 205),
        ("07", 253),
        ("08", 205),
        ("09", 142),
        ("10", 153),
        ("11", 159),
        ("12", 140),
        ("13", 140),
        ("14", 140),
        ("15", 160),
        ("16", 140),
        ("17", 188),
        ("18", 225),
        ("19", 235),
        ("20", 189),
        ("21", 180),
        ("22", 122),
        ("23", 167),
    ];

    let bar_chart_data = price_cell_vec_to_chart_data(&date_matrix[0]);
    let bar_chart_data = chart_data_as_str_ref(&bar_chart_data);
    let bar_chart_max = price_cell_max(&date_matrix[0].cells);

    println!("{}", date_chart_max(sample_data));
    println!("{:?}", sample_data);

    draw(bar_chart_data.as_slice(), bar_chart_max, &date_matrix[0].date)?;
    // draw(sample_data, date_chart_max(sample_data))?;

    Ok(())
}
