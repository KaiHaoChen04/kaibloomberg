use chrono::Local;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Modifier,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Row, Table},
};

use crate::app::{App, OptionsSide};
use crate::app_data::OptionsContractNode;

pub fn draw_options_chart(frame: &mut Frame, app: &mut App, area: Rect) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(4), Constraint::Min(0)])
        .split(area);

    let expiration_label = app
        .current_expiration_timestamp()
        .and_then(|ts| chrono::DateTime::from_timestamp(ts, 0))
        .map(|dt| dt.with_timezone(&Local).format("%Y-%m-%d").to_string())
        .unwrap_or_else(|| "-".to_string());

    let side_label = match app.options_side {
        OptionsSide::Calls => "Calls",
        OptionsSide::Puts => "Puts",
    };

    let header = Paragraph::new(format!(
        "Symbol: {} | Expiration: {} | Side: {}\n{}",
        app.active_label(),
        expiration_label,
        side_label,
        app.options_status
    ))
    .block(Block::default().title(" Options ").borders(Borders::ALL));

    frame.render_widget(header, layout[0]);

    let header_col = Row::new(
        vec!["Last Trade Date", "Strike", "Last Price", "Bid", "Ask", "Volume", "Open Interest", "Implied Volatility"])
        .style(
            Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD)
        )
        .bottom_margin(1);

    let contracts = app
        .options
        .get(app.options_selected_expiration)
        .and_then(|chain| match app.options_side {
            OptionsSide::Calls => chain.calls.as_ref(),
            OptionsSide::Puts => chain.puts.as_ref(),
        });

    let max_rows = layout[1].height.saturating_sub(3) as usize;
    app.options_page_size = max_rows.max(1);
    let max_scroll = contracts
        .map(|items| items.len().saturating_sub(app.options_page_size))
        .unwrap_or(0);
    if app.options_scroll > max_scroll {
        app.options_scroll = max_scroll;
    }

    let rows = build_option_rows(contracts, app.options_scroll, app.options_page_size);

    let widths = [
        Constraint::Percentage(12); 8
    ];

    let table = Table::new(rows, widths)
        .header(header_col)
        .block(Block::default().title(" Options ").borders(Borders::ALL))
        .column_spacing(2);

    frame.render_widget(table, layout[1]);
}

fn build_option_rows(contracts: Option<&Vec<OptionsContractNode>>, scroll: usize, page_size: usize,) -> Vec<Row<'static>> {
    let mut rows = Vec::new();

    if let Some(items) = contracts {
        let start = scroll.min(items.len());
        let end = (start + page_size.max(1)).min(items.len());
        for contract in items[start..end].iter() {
            rows.push(Row::new(vec![
                format_date(contract.last_trade_date),
                format_decimal(contract.strike),
                format_decimal(contract.last_price),
                format_decimal(contract.bid),
                format_decimal(contract.ask),
                format_u64(contract.volume),
                format_u64(contract.open_interest),
                format_iv(contract.implied_volatility),
            ]));
        }
    }

    if rows.is_empty() {
        rows.push(Row::new(vec![
            "No option contracts".to_string(),
            "-".to_string(),
            "-".to_string(),
            "-".to_string(),
            "-".to_string(),
            "-".to_string(),
            "-".to_string(),
            "-".to_string(),
        ]));
    }

    rows
}

fn format_decimal(value: Option<f64>) -> String {
    value
        .map(|v| format!("{v:.2}"))
        .unwrap_or_else(|| "-".to_string())
}

fn format_u64(value: Option<u64>) -> String {
    value
        .map(|v| v.to_string())
        .unwrap_or_else(|| "-".to_string())
}

fn format_iv(value: Option<f64>) -> String {
    value
        .map(|v| format!("{:.2}%", v * 100.0))
        .unwrap_or_else(|| "-".to_string())
}

fn format_date(timestamp: Option<i64>) -> String {
    timestamp
        .and_then(|ts| chrono::DateTime::from_timestamp(ts, 0))
        .map(|dt| dt.with_timezone(&Local).format("%Y-%m-%d").to_string())
        .unwrap_or_else(|| "-".to_string())
}