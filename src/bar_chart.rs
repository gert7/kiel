
use crossterm::{event::DisableMouseCapture, execute, terminal::EnterAlternateScreen};
use rust_decimal::prelude::ToPrimitive;
use tui::{
    backend::CrosstermBackend,
    style::{Color, Style},
    widgets::{BarChart, Block, Borders},
    Terminal,
};

use crate::price_matrix::{DateColumn, PriceCell};

// fn date_chart_max(cells: &[(&str, u64)]) -> u64 {
//     let prices = cells.iter().map(|c| c.1);
//     prices.max().unwrap()
// }

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

fn date_chart_max(cells: &Vec<PriceCell>) -> u64 {
    let prices = cells.iter().map(|c| c.price.to_u64().unwrap());
    prices.max().unwrap()
}

pub fn draw(date: &DateColumn) -> color_eyre::Result<()> {
    let bar_chart_max = date_chart_max(&date.cells);
    let title = date.date.as_ref();

    let bar_chart_data = price_cell_vec_to_chart_data(date);
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