use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Modifier,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Row, Table},
};

pub fn draw_options_chart(frame: &Frame, area: Rect) {
    let call_puts_box = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(area);

    

}