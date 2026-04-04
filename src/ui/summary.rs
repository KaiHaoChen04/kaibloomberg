use ratatui::{Frame, 
    layout::{Constraint}, 
    style::Modifier, 
    widgets::{Block, Borders, Paragraph, Row, Table},
    style::{Style, Color}};

use crate::app::App;

pub fn draw_summary_box(frame: &mut Frame, area: ratatui::layout::Rect) {

    let header_col = Row::new(vec!["Ticker", "Market $", "Avg $"])
        .style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .bottom_margin(1);
    
    //Placeholder for now
    let rows = vec![
        Row::new(vec!["AAPL", "$189.42", "$150.00"]),
        Row::new(vec!["TSLA", "$245.10", "$210.50"]),
        Row::new(vec!["NVDA", "$875.00", "$600.00"]),
    ];

    let widths = [
        Constraint::Percentage(33),
        Constraint::Percentage(33),
        Constraint::Percentage(34),
    ];

    let table = Table::new(rows, widths)
        .header(header_col)
        .block(Block::default().title(" Portfolio ").borders(Borders::ALL))
        .column_spacing(2);

    frame.render_widget(table, area);

}
pub fn draw_alternative_footer<'a>() -> Paragraph<'a> {

    let footer_note = "q quit | a add ticker | d delete ticker | tab chart";

    let footer =
        Paragraph::new(footer_note).block(Block::default().title(" Controls ").borders(Borders::ALL));

    footer
}
pub fn draw_main_footer<'a>(app: &App) -> Paragraph<'a> {
    let hint = if app.input_mode {
        format!(
            "Add ticker: {} | Enter confirm | Esc cancel",
            app.input_buffer
        )
    } 
    else {
        "q quit | ←/→ headers | a add stock | d remove stock | t header/portfolio | l line | c candle | ^/v portfolio | tab holdings"
            .to_string()
    };
    
    let footer =
        Paragraph::new(hint).block(Block::default().title(" Controls ").borders(Borders::ALL));

    footer
}