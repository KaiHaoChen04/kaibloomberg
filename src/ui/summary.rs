use ratatui::{Frame, 
    layout::{Layout, Constraint, Direction},
    widgets::{Paragraph, Block, Borders}};

use crate::app::App;

pub fn draw_summary_box(app: &mut App, frame: &mut Frame, area: ratatui::layout::Rect) {

    let root = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),
        ])
        .split(frame.area());
            
}
pub fn draw_alternative_footer<'a>() -> Paragraph<'a> {

    let footer_note = "q quit | a add ticker | d delete ticker | tab switch views";

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