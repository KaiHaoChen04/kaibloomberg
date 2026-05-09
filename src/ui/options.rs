use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Modifier,
    style::{Color, Style},
    widgets::{Block, Borders, Row, Table},
};

pub fn draw_options_chart(frame: &mut Frame, area: Rect) {

    let header_col = Row::new(
        vec!["Last Trade Date", "Strike", "Last Price", "Bid", "Ask", "Volume", "Open Interest", "Implied Volatility"])
        .style(
            Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD)
        )
        .bottom_margin(1);

    let rows = vec![
        Row::new(vec!["-", "-", "-", "-", "-", "-", "-", "-"])];

    let widths = [
        Constraint::Percentage(12); 8
    ];

    let table = Table::new(rows, widths)
        .header(header_col)
        .block(Block::default().title(" Options ").borders(Borders::ALL))
        .column_spacing(2);

    frame.render_widget(table, area);
}