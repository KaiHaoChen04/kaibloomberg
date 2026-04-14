use ratatui::{
    Frame,
    layout::Constraint,
    style::Modifier,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Row, Table},
};

use crate::{
    app::{App, CurrentScreen},
    app_data::Holdings,
};

pub fn draw_summary_box(frame: &mut Frame, list: &Holdings, area: ratatui::layout::Rect) {
    let header_col = Row::new(vec!["Ticker", "Quantity", "Avg $"])
        .style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .bottom_margin(1);

    let rows: Vec<Row> = if list.holding_list.is_empty() {
        vec![Row::new(vec!["(empty)", "-", "-"])]
    } else {
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
        Constraint::Percentage(15),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
    ];

    let table = Table::new(rows, widths)
        .header(header_col)
        .block(Block::default().title(" Portfolio ").borders(Borders::ALL))
        .column_spacing(2);

    frame.render_widget(table, area);
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
    } else {
        idle_hint.to_string()
    };

    Paragraph::new(hint).block(Block::default().title(" Controls ").borders(Borders::ALL))
}
