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
pub fn draw_alternative_footer(frame: &mut Frame, area: ratatui::layout::Rect) {

    let footer_note = "q quit | a add ticker | d delete ticker | tab switch views";

    let footer =
        Paragraph::new(footer_note).block(Block::default().title(" Controls ").borders(Borders::ALL));
    frame.render_widget(footer, area);

}