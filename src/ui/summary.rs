use ratatui::{Frame, layout::{Layout, Constraint, Direction}};

use crate::app::App;

pub fn draw_summary_box(app: &mut App, frame: &mut Frame) {

    let root = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0)
        ])
        .split(frame.area());


    
}