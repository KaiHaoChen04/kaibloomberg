use ratatui::{
    Frame,
    layout::{Constraint, Layout, Direction, Rect},
    style::Modifier,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Row, Table},
};

use tui_piechart::{PieChart, PieSlice};

use crate::{
    app::{App, CurrentScreen},
    app_data::Holdings,
};

pub fn draw_summary_box(frame: &mut Frame, list: &Holdings, area: Rect) {

    let main_box = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(area);


    let header_col = Row::new(vec!["Ticker", "Quantity", "Avg $"])
        .style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .bottom_margin(1);

    let rows: Vec<Row> = if list.holding_list.is_empty() {
        vec![Row::new(vec!["(empty)", "-", "-"])]
    }
    else {
        list.holding_list
            .iter()
            .map(|(symbol, stock)| {
                Row::new(vec![
                    symbol.clone(),
                    format!("{:.2}", stock.get_quantity()),
                    format!("{:.2}", stock.get_avg_price()),
                ])
            })
            .collect()
    };

    let widths = [
        Constraint::Percentage(33),
        Constraint::Percentage(33),
        Constraint::Percentage(33),
    ];

    let table = Table::new(rows, widths)
        .header(header_col)
        .block(Block::default().title(" Portfolio ").borders(Borders::ALL))
        .column_spacing(2);

    let total: f64 = list.holding_list.values()
        .map(|v| v.get_avg_price() * v.get_quantity())
        .sum();

    let slices = if list.holding_list.is_empty() {
        vec![PieSlice::new("None", 100.0, Color::White)]
    }
    else {
        list.holding_list
            .iter()
            .map(|(symbol, stock)| {
                PieSlice::new(
                    symbol.as_str(), 
                    stock.get_avg_price() / total, 
                    Color::Blue,
                )
            })
            .collect() 
    };

    let pie_chart = PieChart::new(slices)
        .show_legend(true)
        .show_percentages(true);

    frame.render_widget(table, main_box[0]);
    frame.render_widget(pie_chart, main_box[1]);
}

pub fn draw_footer<'a>(app: &App, idle_hint: &'a str) -> Paragraph<'a> {
    let hint = if app.input_mode {
        match app.current_screen {
            CurrentScreen::Main => {
                format!(
                    "Add ticker: {} | Enter confirm | Esc cancel",
                    app.input_buffer
                )
            }
            CurrentScreen::Portfolio => {
                format!(
                    "{}: {} | Enter confirm | Esc cancel",
                    app.portfolio_input_label(),
                    app.port_buffer
                )
            }
        }
    }
    else {
        idle_hint.to_string()
    };

    Paragraph::new(hint).block(Block::default().title(" Controls ").borders(Borders::ALL))
}