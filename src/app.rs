use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use crossterm::event::{KeyCode, KeyEvent};

use crate::app_data::{Candle, Headers, Holdings, Range, fetch_candles};
use crate::utils::{sanitize_symbol, status_cached, status_failed, status_loading, status_updated};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChartMode {
    Line,
    Candle,
}

#[derive(PartialEq)]
pub enum CurrentScreen {
    Main,
    Portfolio,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PortfolioInputStep {
    Ticker,
    AveragePrice,
    Quantity,
}

pub enum FetchResult {
    Success {
        symbol: String,
        candles: Vec<Candle>,
    },
    Error {
        symbol: String,
        error: String,
    },
}

pub struct App {
    pub selected_header: usize,
    pub portfolio: Vec<String>,
    pub holdings: Holdings,
    pub selected_portfolio: usize,
    pub use_portfolio_symbol: bool,
    pub input_mode: bool,
    pub input_buffer: String,
    pub port_buffer: String,
    portfolio_input_step: PortfolioInputStep,
    portfolio_pending_ticker: String,
    portfolio_pending_avg_price: Option<f64>,
    pub chart_mode: ChartMode,
    pub candles: Vec<Candle>,
    pub cache: HashMap<String, Vec<Candle>>,
    pub status: String,
    pub is_loading: bool,
    pub pending_symbol: Option<String>,
    pub should_quit: bool,
    pub last_refresh: Instant,
    pub refresh_interval: Duration,
    pub current_screen: CurrentScreen,
}

impl App {
    pub fn new() -> Self {
        Self {
            selected_header: 0,
            portfolio: vec![],
            holdings: Holdings::default(),
            selected_portfolio: 0,
            use_portfolio_symbol: false,
            input_mode: false,
            input_buffer: String::new(),
            port_buffer: String::new(),
            portfolio_input_step: PortfolioInputStep::Ticker,
            portfolio_pending_ticker: String::new(),
            portfolio_pending_avg_price: None,
            chart_mode: ChartMode::Line,
            candles: Vec::new(),
            cache: HashMap::new(),
            status: "Loading market data...".to_string(),
            is_loading: false,
            pending_symbol: None,
            should_quit: false,
            last_refresh: Instant::now() - Duration::from_secs(30),
            refresh_interval: Duration::from_millis(1000),
            current_screen: CurrentScreen::Main,
        }
    }

    pub fn header_tabs(&self) -> [Headers; 5] {
        Headers::all()
    }

    pub fn current_header(&self) -> Headers {
        self.header_tabs()[self.selected_header]
    }

    pub fn active_symbol(&self) -> String {
        if self.use_portfolio_symbol {
            if let Some(symbol) = self.portfolio.get(self.selected_portfolio) {
                return symbol.clone();
            }
        }

        self.current_header().symbol().to_string()
    }
    pub fn active_label(&self) -> String {
        if self.use_portfolio_symbol {
            if let Some(label) = self.portfolio.get(self.selected_portfolio) {
                return label.clone();
            }
        }
        self.current_header().label().to_string()
    }

    pub fn active_symbol_source(&self) -> &'static str {
        if self.use_portfolio_symbol {
            "Portfolio"
        }
        else {
            "Header"
        }
    }

    pub fn line_points(&self) -> Vec<(f64, f64)> {
        self.candles
            .iter()
            .enumerate()
            .map(|(i, c)| (i as f64, c.close))
            .collect()
    }

    pub fn should_refresh(&self) -> bool {
        self.last_refresh.elapsed() >= self.refresh_interval
    }

    pub fn schedule_refresh(&mut self) -> Option<String> {
        if self.is_loading {
            return None;
        }

        let symbol = self.active_symbol();
        self.is_loading = true;
        self.pending_symbol = Some(symbol.clone());
        self.status = status_loading(&symbol);
        Some(symbol)
    }

    pub fn on_fetch_result(&mut self, message: FetchResult) {
        match message {
            FetchResult::Success { symbol, candles } => {
                self.cache.insert(symbol.clone(), candles.clone());
                if symbol == self.active_symbol() {
                    self.candles = candles;
                }
                if self.pending_symbol.as_deref() == Some(symbol.as_str()) {
                    self.is_loading = false;
                    self.pending_symbol = None;
                    self.last_refresh = Instant::now();
                    let count = self
                        .cache
                        .get(&symbol)
                        .map(|values| values.len())
                        .unwrap_or(0);
                    self.status = status_updated(&symbol, count);
                }
            }
            FetchResult::Error { symbol, error } => {
                if self.pending_symbol.as_deref() == Some(symbol.as_str()) {
                    self.is_loading = false;
                    self.pending_symbol = None;
                    self.last_refresh = Instant::now();
                    self.status = status_failed(&symbol, &error);
                }
            }
        }
    }

    pub fn refresh_symbol(symbol: String) -> impl std::future::Future<Output = FetchResult> {
        async move {
            match fetch_candles(&symbol, Range::Day).await {
                Ok(candles) => FetchResult::Success { symbol, candles },
                Err(error) => FetchResult::Error { symbol, error },
            }
        }
    }

    fn show_cached_or_loading(&mut self) {
        let symbol = self.active_symbol();
        if let Some(cached) = self.cache.get(&symbol) {
            self.candles = cached.clone();
            self.status = status_cached(&symbol, self.candles.len());
        }
        else {
            self.status = status_loading(&symbol);
        }
    }

    fn reset_portfolio_input(&mut self) {
        self.port_buffer.clear();
        self.portfolio_input_step = PortfolioInputStep::Ticker;
        self.portfolio_pending_ticker.clear();
        self.portfolio_pending_avg_price = None;
    }

    pub fn portfolio_input_label(&self) -> &'static str {
        match self.portfolio_input_step {
            PortfolioInputStep::Ticker => "Ticker",
            PortfolioInputStep::AveragePrice => "Average price",
            PortfolioInputStep::Quantity => "Quantity",
        }
    }

    pub fn on_key(&mut self, key: KeyEvent) -> bool {
        if self.input_mode {
            return self.handle_input_mode(key);
        }

        match key.code {
            KeyCode::Char('q') => {
                self.should_quit = true;
                true
            }
            KeyCode::Left => {
                if self.selected_header == 0 {
                    self.selected_header = self.header_tabs().len() - 1;
                }
                else {
                    self.selected_header -= 1;
                }
                self.use_portfolio_symbol = false;
                self.show_cached_or_loading();
                true
            }
            KeyCode::Right => {
                self.selected_header = (self.selected_header + 1) % self.header_tabs().len();
                self.use_portfolio_symbol = false;
                self.show_cached_or_loading();
                true
            }
            KeyCode::Char('a') => {
                self.input_mode = true;
                match self.current_screen {
                    CurrentScreen::Main => {
                        self.input_buffer.clear();
                        self.status = "Input mode: type ticker and press Enter".to_string();
                    }
                    CurrentScreen::Portfolio => {
                        self.reset_portfolio_input();
                        self.status = "Input mode: type ticker, then average price, then quantity"
                            .to_string();
                    }
                }
                false
            }
            KeyCode::Char('d') => {
                if self.portfolio.is_empty() {
                    self.status = "Portfolio is already empty".to_string();
                    false
                }
                else {
                    let removed = self.portfolio.remove(self.selected_portfolio);
                    if self.selected_portfolio >= self.portfolio.len() && !self.portfolio.is_empty()
                    {
                        self.selected_portfolio = self.portfolio.len() - 1;
                    }
                    if self.portfolio.is_empty() {
                        self.use_portfolio_symbol = false;
                        self.selected_portfolio = 0;
                    }
                    self.show_cached_or_loading();
                    self.status = format!("Removed {} from portfolio", removed);
                    true
                }
            }
            KeyCode::Char('j') | KeyCode::Down => {
                if !self.portfolio.is_empty() {
                    let previous = self.selected_portfolio;
                    self.selected_portfolio = (self.selected_portfolio + 1) % self.portfolio.len();
                    if self.use_portfolio_symbol && previous != self.selected_portfolio {
                        self.show_cached_or_loading();
                        return true;
                    }
                }
                false
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if !self.portfolio.is_empty() {
                    let previous = self.selected_portfolio;
                    if self.selected_portfolio == 0 {
                        self.selected_portfolio = self.portfolio.len() - 1;
                    }
                    else {
                        self.selected_portfolio -= 1;
                    }
                    if self.use_portfolio_symbol && previous != self.selected_portfolio {
                        self.show_cached_or_loading();
                        return true;
                    }
                }
                false
            }
            KeyCode::Char('t') => {
                if self.portfolio.is_empty() {
                    self.status = "Portfolio is empty, using header symbols".to_string();
                    false
                }
                else {
                    self.use_portfolio_symbol = !self.use_portfolio_symbol;
                    self.show_cached_or_loading();
                    true
                }
            }
            KeyCode::Char('l') => {
                self.chart_mode = ChartMode::Line;
                self.status = "Switched to line chart".to_string();
                false
            }
            KeyCode::Char('c') => {
                self.chart_mode = ChartMode::Candle;
                self.status = "Switched to candle view".to_string();
                false
            }
            KeyCode::Tab => match self.current_screen {
                CurrentScreen::Main => {
                    self.current_screen = CurrentScreen::Portfolio;
                    false
                }
                CurrentScreen::Portfolio => {
                    self.current_screen = CurrentScreen::Main;
                    true
                }
            },
            _ => false,
        }
    }

    fn handle_input_mode(&mut self, key: KeyEvent) -> bool {
        match self.current_screen {
            CurrentScreen::Main => match key.code {
                KeyCode::Esc => {
                    self.input_mode = false;
                    self.status = "Input canceled".to_string();
                    false
                }
                KeyCode::Enter => {
                    let symbol = sanitize_symbol(&self.input_buffer);
                    self.input_mode = false;

                    if symbol.is_empty() {
                        self.status = "Ticker cannot be empty".to_string();
                        return false;
                    }

                    if !self.portfolio.iter().any(|existing| existing == &symbol) {
                        self.portfolio.push(symbol.clone());
                        self.selected_portfolio = self.portfolio.len() - 1;
                    }
                    else if let Some(index) = self
                        .portfolio
                        .iter()
                        .position(|existing| existing == &symbol)
                    {
                        self.selected_portfolio = index;
                    }

                    self.use_portfolio_symbol = true;
                    self.input_buffer.clear();
                    self.show_cached_or_loading();
                    true
                }
                KeyCode::Backspace => {
                    self.input_buffer.pop();
                    false
                }
                KeyCode::Char(ch) => {
                    self.input_buffer.push(ch);
                    false
                }
                _ => false,
            },
            CurrentScreen::Portfolio => match key.code {
                KeyCode::Esc => {
                    self.input_mode = false;
                    self.reset_portfolio_input();
                    self.status = "Input canceled".to_string();
                    false
                }
                KeyCode::Char(ch) => {
                    self.port_buffer.push(ch);
                    false
                }
                KeyCode::Backspace => {
                    self.port_buffer.pop();
                    false
                }
                KeyCode::Enter => {
                    let raw = self.port_buffer.trim();
                    match self.portfolio_input_step {
                        PortfolioInputStep::Ticker => {
                            let symbol = sanitize_symbol(raw);
                            if symbol.is_empty() {
                                return false;
                            }

                            self.portfolio_pending_ticker = symbol;
                            self.portfolio_input_step = PortfolioInputStep::AveragePrice;
                            self.port_buffer.clear();
                            false
                        }
                        PortfolioInputStep::AveragePrice => {
                            match raw.parse::<f64>() {
                                Ok(value) if value >= 0.0 => {
                                    self.portfolio_pending_avg_price = Some(value);
                                    self.portfolio_input_step = PortfolioInputStep::Quantity;
                                    self.port_buffer.clear();
                                }
                                _ => (),
                            }
                            false
                        }
                        PortfolioInputStep::Quantity => {
                            let quantity = match raw.parse::<f64>() {
                                Ok(value) if value >= 0.0 => value,
                                _ => 0.0,
                            };

                            let symbol = self.portfolio_pending_ticker.clone();
                            let average_price =
                                self.portfolio_pending_avg_price.unwrap_or_default();

                            self.holdings
                                .upsert(symbol.clone(), average_price, quantity);

                            if !self.portfolio.iter().any(|existing| existing == &symbol) {
                                self.portfolio.push(symbol.clone());
                                self.selected_portfolio = self.portfolio.len() - 1;
                            }
                            else if let Some(index) = self
                                .portfolio
                                .iter()
                                .position(|existing| existing == &symbol)
                            {
                                self.selected_portfolio = index;
                            }

                            self.use_portfolio_symbol = true;
                            self.input_mode = false;
                            self.reset_portfolio_input();
                            true
                        }
                    }
                }
                _ => false,
            },
        }
    }
}
