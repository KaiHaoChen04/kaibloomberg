use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
};

use crate::{
    app::{App, CurrentScreen},
    app_data::News,
};

pub fn draw_news(frame: &mut Frame, app: &mut App, area: Rect) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(4), Constraint::Min(0)])
        .split(area);

    let sources: Vec<Line> = News::all()
        .iter()
        .map(|source| {
            let label = format!(" {} ", source.label());
            if *source == app.news_source {
                Line::from(Span::styled(
                    label,
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ))
            }
            else {
                Line::from(label)
            }
        })
        .collect();

    let sources_box = Paragraph::new(sources)
        .block(Block::default().title(" Sources ").borders(Borders::ALL))
        .alignment(Alignment::Center);

    let header = Paragraph::new(Line::from(vec![
        Span::styled("News: ", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(app.news_source.label()),
        Span::raw(" | "),
        Span::raw(app.news_status.clone()),
    ]))
    .block(Block::default().title(" News ").borders(Borders::ALL))
    .wrap(Wrap { trim: true });

    let top = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(32), Constraint::Min(0)])
        .split(layout[0]);

    frame.render_widget(sources_box, top[0]);
    frame.render_widget(header, top[1]);

    let max_rows = layout[1].height.saturating_sub(2) as usize;
    app.news_page_size = max_rows.max(1);
    let max_scroll = app
        .news_items
        .len()
        .saturating_sub(app.news_page_size.max(1));
    if app.news_scroll > max_scroll {
        app.news_scroll = max_scroll;
    }

    let items = build_news_items(app, app.news_scroll, app.news_page_size);
    let mut state = ListState::default();
    if !items.is_empty() && !app.news_items.is_empty() {
        state.select(Some(0));
    }

    let list = List::new(items)
        .block(Block::default().title(" Headlines ").borders(Borders::ALL))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

    frame.render_stateful_widget(list, layout[1], &mut state);

    if app.current_screen == CurrentScreen::News && app.news_items.is_empty() {
        let empty = Paragraph::new("No news items available yet")
            .block(Block::default().borders(Borders::ALL))
            .wrap(Wrap { trim: true });
        frame.render_widget(empty, layout[1]);
    }
}

fn build_news_items(app: &App, scroll: usize, page_size: usize) -> Vec<ListItem<'static>> {
    let start = scroll.min(app.news_items.len());
    let end = (start + page_size.max(1)).min(app.news_items.len());

    let mut rows = Vec::new();
    for item in &app.news_items[start..end] {
        let mut lines = Vec::new();
        lines.push(Line::from(Span::styled(
            item.title.clone(),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )));

        if let Some(pub_date) = &item.pub_date {
            lines.push(Line::from(Span::styled(
                pub_date.clone(),
                Style::default().fg(Color::DarkGray),
            )));
        }

        if let Some(description) = &item.description {
            lines.push(Line::from(Span::raw(description.clone())));
        }

        lines.push(Line::from(Span::styled(
            item.link.clone(),
            Style::default().fg(Color::Cyan),
        )));

        rows.push(ListItem::new(lines));
    }

    if rows.is_empty() {
        rows.push(ListItem::new("No usable headlines found"));
    }

    rows
}
