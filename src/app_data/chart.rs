use serde::Deserialize;

use crate::utils::{encode_symbol, value_or_fallback};

#[derive(Clone, Copy, Debug)]
pub enum Range {
    Day,
}

impl Range {
    pub fn as_query(self) -> &'static str {
        match self {
            Self::Day => "1d",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Candle {
    pub ts: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
}

#[derive(Deserialize)]
struct YahooChartResponse {
    chart: YahooChart,
}

#[derive(Deserialize)]
struct YahooChart {
    result: Option<Vec<YahooResult>>,
    error: Option<YahooError>,
}

#[derive(Deserialize)]
struct YahooError {
    description: Option<String>,
}

#[derive(Deserialize)]
struct YahooResult {
    timestamp: Option<Vec<i64>>,
    indicators: YahooIndicators,
}

#[derive(Deserialize)]
struct YahooIndicators {
    quote: Vec<YahooQuote>,
}

#[derive(Deserialize)]
struct YahooQuote {
    open: Option<Vec<Option<f64>>>,
    high: Option<Vec<Option<f64>>>,
    low: Option<Vec<Option<f64>>>,
    close: Option<Vec<Option<f64>>>,
}

