use color_eyre::eyre::{eyre, self};
use chrono::{Timelike, Datelike};
use crossterm::{event::DisableMouseCapture, execute, terminal::EnterAlternateScreen};
use rust_decimal::prelude::ToPrimitive;
use tui::{
    backend::CrosstermBackend,
    style::{Color, Style},
    widgets::{BarChart, Block, Borders},
    Terminal,
};

use crate::price_matrix::{DateColumn, PriceCell, DaySlice};

// fn date_chart_max(cells: &[(&str, u64)]) -> u64 {
//     let prices = cells.iter().map(|c| c.1);
//     prices.max().unwrap()
// }

fn price_cell_vec_to_chart_data(dc: &Vec<PriceCell>) -> Vec<(String, u64)> {
    let mut result = vec![];
    for cell in dc {
        let label = format!("{:02}", cell.moment.hour());
        let value = cell.price.0.to_u64().unwrap();
        result.push((label, value));
    }
    result
}

/// Passion function for converting data for tui
fn chart_data_as_str_ref<'a>(invec: &'a Vec<(String, u64)>) -> Vec<(&'a str, u64)> {
    let mut result = vec![];
    for item in invec {
        result.push((item.0.as_ref(), item.1));
    }
    result
}

fn date_chart_max(cells: &Vec<PriceCell>) -> u64 {
    let prices = cells.iter().map(|c| c.price.0.to_u64().unwrap());
    prices.max().unwrap()
}

pub fn draw(slice: &Vec<PriceCell>) -> eyre::Result<()> {
    let bar_chart_max = date_chart_max(slice);
    let first_dt = slice.get(0).ok_or(eyre!("PriceCell vector is empty!"))?.moment;
    let title = format!("{}-{}-{}", first_dt.year(), first_dt.month(), first_dt.day());

    let bar_chart_data = price_cell_vec_to_chart_data(&slice);
    let bar_chart_data = chart_data_as_str_ref(&bar_chart_data);

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
            .data(bar_chart_data.as_slice())
            .max(bar_chart_max);
        f.render_widget(chart, size);
    })?;

    Ok(())
}