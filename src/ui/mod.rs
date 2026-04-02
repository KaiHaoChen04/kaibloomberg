use std::{error::Error, io, time::Duration};
use chrono::{Local};

use crossterm::{
    event::{self, Event, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    symbols,
    text::Line as TextLine,
    widgets::{
        Axis, Block, Borders, Chart, Dataset, GraphType, List, ListItem, Paragraph, Tabs, Wrap,
        canvas::{Canvas, Line as CanvasLine, Rectangle},
    },
};
use tokio::sync::mpsc;

use crate::app::{App, ChartMode, FetchResult, CurrentScreen};
mod summary;

pub async fn run_ui(app: &mut App) -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let (result_tx, mut result_rx) = mpsc::unbounded_channel::<FetchResult>();

    if let Some(symbol) = app.schedule_refresh() {
        spawn_refresh(symbol, result_tx.clone());
    }

    loop {
        while let Ok(message) = result_rx.try_recv() {
            app.on_fetch_result(message);
        }

        terminal.draw(|frame| draw(frame, app))?;

        if app.should_quit {
            break;
        }

        if app.should_refresh() {
            if let Some(symbol) = app.schedule_refresh() {
                spawn_refresh(symbol, result_tx.clone());
            }
        }

        if event::poll(Duration::from_millis(100))?
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            let should_refresh = app.on_key(key);
            if should_refresh {
                if let Some(symbol) = app.schedule_refresh() {
                    spawn_refresh(symbol, result_tx.clone());
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

fn spawn_refresh(symbol: String, result_tx: mpsc::UnboundedSender<FetchResult>) {
    tokio::spawn(async move {
        let message = App::refresh_symbol(symbol).await;
        let _ = result_tx.send(message);
    });
}

fn draw(frame: &mut Frame, app: &App) {
    let root = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let top = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(15),
        ])
        .split(root[0]);

    let headers = app.header_tabs();
    let tab_labels: Vec<TextLine> = headers
        .iter()
        .map(|header| TextLine::from(format!(" {} ", header.label())))
        .collect();

    let tabs = Tabs::new(tab_labels)
        .select(app.selected_header)
        .block(
            Block::default()
                .title(" Market Headers ")
                .borders(Borders::ALL),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        );
    let live_time = Local::now().format("%H:%M:%S").to_string();
    let time_box = Paragraph::new(live_time)
        .block(Block::default().title("")
        .borders(Borders::ALL))
        .alignment(Alignment::Center);
    frame.render_widget(time_box, top[1]);
    frame.render_widget(tabs, top[0]);

    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(30), Constraint::Min(0)])
        .split(root[1]);

    draw_left_panel(frame, app, body[0]);
    draw_chart(frame, app, body[1]);

    let hint = if app.input_mode {
        format!(
            "Add ticker: {} | Enter confirm | Esc cancel",
            app.input_buffer
        )
    } 
    else {
        "q quit | ←/→ headers | a add stock | d remove stock | t header/portfolio | l line | c candle | ^/v portfolio | h holdings"
            .to_string()
    };
    let footer =
        Paragraph::new(hint).block(Block::default().title(" Controls ").borders(Borders::ALL));
    frame.render_widget(footer, root[2]);
}

fn draw_left_panel(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let left = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(8),
            Constraint::Length(6),
            Constraint::Length(4),
        ])
        .split(area);

    let items: Vec<ListItem> = if app.portfolio.is_empty() {
        vec![ListItem::new("(empty)")]
    } 
    else {
        app.portfolio
            .iter()
            .map(|symbol| ListItem::new(symbol.clone()))
            .collect()
    };

    let mut state = ratatui::widgets::ListState::default();
    if !app.portfolio.is_empty() {
        state.select(Some(app.selected_portfolio));
    }

    let title = if app.use_portfolio_symbol {
        " Portfolio [ACTIVE] "
    } 
    else {
        " Portfolio "
    };

    let portfolio = List::new(items)
        .block(Block::default().title(title).borders(Borders::ALL))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");
    frame.render_stateful_widget(portfolio, left[0], &mut state);

    let active = Paragraph::new(format!(
        "Source: {}\nSymbol: {}\nMode: {:?}",
        app.active_symbol_source(),
        app.active_label(),
        app.chart_mode
    ))
    .block(Block::default().title(" Session ").borders(Borders::ALL));
    frame.render_widget(active, left[1]);

    let status = Paragraph::new(app.status.clone())
        .block(Block::default().title(" Status ").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    frame.render_widget(status, left[2]);
}

fn draw_chart(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    match app.chart_mode {
        ChartMode::Line => draw_line_chart(frame, app, area),
        ChartMode::Candle => draw_candle_chart(frame, app, area),
    }
}

fn draw_line_chart(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let points = app.line_points();
    if points.is_empty() {
        let empty = Paragraph::new(format!(
            "No data yet for {}\n{}",
            app.active_label(),
            app.status
        ))
        .block(
            Block::default()
                .title(" Live Line Chart ")
                .borders(Borders::ALL),
        );
        frame.render_widget(empty, area);
        return;
    }

    let min_y = points
        .iter()
        .map(|(_, y)| *y)
        .fold(f64::INFINITY, |acc, y| acc.min(y));
    let max_y = points
        .iter()
        .map(|(_, y)| *y)
        .fold(f64::NEG_INFINITY, |acc, y| acc.max(y));
    let mid_y = (min_y + max_y) / 2.0;

    let color = match (app.candles.first(), app.candles.last()) {
        (Some(first), Some(last)) => {
            if first.open > last.close {
                Color::Red
            } 
            else {
                Color::Green
            }
        }
        _ => Color::Green,
    };

    let dataset = Dataset::default()
        .name(app.active_label())
        .marker(symbols::Marker::Braille)
        .style(Style::default().fg(color))
        .graph_type(GraphType::Line)
        .data(&points);

    let open_time = app.candles.first()
        .map(|candle| {
            chrono::DateTime::from_timestamp(candle.ts, 0)
            .map(|dt| dt.with_timezone(&Local).format("%-H:%M").to_string())
            .unwrap_or_else(|| "9:30".to_string())
        })
        .unwrap_or_else(|| "9:30".to_string());


    let chart = Chart::new(vec![dataset])
        .block(
            Block::default()
                .title(" Live Line Chart ")
                .borders(Borders::ALL),
        )
        .x_axis(
            Axis::default()
                .title("Time")
                .bounds([0.0, points.len() as f64])
                .labels(vec![TextLine::from(open_time), TextLine::from("Now")]),
        )
        .y_axis(
            Axis::default()
                .title("Price")
                .bounds([min_y, max_y])
                .labels([
                    TextLine::from(format!("{min_y:.2}")),
                    TextLine::from(format!("{mid_y:.2}")),
                    TextLine::from(format!("{max_y:.2}")),
                ]),
        );

    frame.render_widget(chart, area);
}

fn draw_candle_chart(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    if app.candles.is_empty() {
        let empty = Paragraph::new(format!(
            "No data yet for {}\n{}",
            app.active_symbol(),
            app.status
        ))
        .block(
            Block::default()
                .title(" Candle View ")
                .borders(Borders::ALL),
        );
        frame.render_widget(empty, area);
        return;
    }

    let max_candles = ((area.width as usize).saturating_sub(10) / 2).clamp(12, 120);
    let start = app.candles.len().saturating_sub(max_candles);
    let visible = &app.candles[start..];

    let mut min_y = visible
        .iter()
        .map(|c| c.low)
        .fold(f64::INFINITY, |acc, y| acc.min(y));
    let mut max_y = visible
        .iter()
        .map(|c| c.high)
        .fold(f64::NEG_INFINITY, |acc, y| acc.max(y));

    if (max_y - min_y).abs() < f64::EPSILON {
        min_y -= 1.0;
        max_y += 1.0;
    }

    let candle_count = visible.len() as f64;
    let min_body_height = (max_y - min_y) * 0.002;

    let chart = Canvas::default()
        .block(
            Block::default()
                .title(" Candlestick Chart ")
                .borders(Borders::ALL),
        )
        .x_bounds([0.0, candle_count])
        .y_bounds([min_y, max_y])
        .paint(|ctx| {
            for (idx, candle) in visible.iter().enumerate() {
                let x = idx as f64 + 0.5;
                let color = if candle.close > candle.open {
                    Color::Green
                } else if candle.close < candle.open {
                    Color::Red
                } else {
                    Color::Gray
                };

                ctx.draw(&CanvasLine {
                    x1: x,
                    y1: candle.low,
                    x2: x,
                    y2: candle.high,
                    color,
                });

                let body_bottom = candle.open.min(candle.close);
                let body_top = candle.open.max(candle.close);
                let body_height = (body_top - body_bottom).max(min_body_height);

                ctx.draw(&Rectangle {
                    x: x - 0.35,
                    y: body_bottom,
                    width: 0.7,
                    height: body_height,
                    color,
                });
            }
        });

    frame.render_widget(chart, area);
}
