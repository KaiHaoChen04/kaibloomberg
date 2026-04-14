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

pub async fn fetch_candles(symbol: &str, range: Range) -> Result<Vec<Candle>, String> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36")
        .build()
        .map_err(|e| format!("HTTP client error: {e}"))?;

    let encoded_symbol = encode_symbol(symbol);
    let mut last_error = String::new();

    for host in ["query1.finance.yahoo.com", "query2.finance.yahoo.com"] {
        for interval in ["1m", "5m", "15m"] {
            let url = format!(
                "https://{host}/v8/finance/chart/{encoded_symbol}?interval={interval}&range={}",
                range.as_query()
            );

            let response = match client.get(&url).send().await {
                Ok(response) => response,
                Err(err) => {
                    last_error = format!("Network error: {err}");
                    continue;
                }
            };

            let status = response.status();
            let body = match response.text().await {
                Ok(text) => text,
                Err(err) => {
                    last_error = format!("Failed reading response body: {err}");
                    continue;
                }
            };

            if !status.is_success() {
                let preview: String = body.chars().take(100).collect();
                last_error = format!("Upstream status {}: {}", status, preview);
                continue;
            }

            let parsed: YahooChartResponse = match serde_json::from_str(&body) {
                Ok(parsed) => parsed,
                Err(err) => {
                    last_error = format!("Invalid JSON payload: {err}");
                    continue;
                }
            };

            if let Some(error) = parsed.chart.error
                && let Some(description) = error.description
            {
                last_error = description;
                continue;
            }

            let result = match parsed
                .chart
                .result
                .and_then(|items| items.into_iter().next())
            {
                Some(result) => result,
                None => {
                    last_error = "No chart result returned for symbol".to_string();
                    continue;
                }
            };

            let timestamps = match result.timestamp {
                Some(timestamps) => timestamps,
                None => {
                    last_error = "Missing timestamp series".to_string();
                    continue;
                }
            };

            let quote = match result.indicators.quote.into_iter().next() {
                Some(quote) => quote,
                None => {
                    last_error = "Missing quote data".to_string();
                    continue;
                }
            };

            let close = match quote.close {
                Some(values) => values,
                None => {
                    last_error = "Missing close values".to_string();
                    continue;
                }
            };

            let open = quote.open;
            let high = quote.high;
            let low = quote.low;

            let mut candles = Vec::with_capacity(timestamps.len());
            for (i, ts) in timestamps.iter().enumerate() {
                let close_value = close.get(i).and_then(|value| *value);
                let Some(close_value) = close_value
                else {
                    continue;
                };

                let open_value = value_or_fallback(open.as_ref(), i, close_value);
                let high_value = value_or_fallback(high.as_ref(), i, close_value);
                let low_value = value_or_fallback(low.as_ref(), i, close_value);

                candles.push(Candle {
                    ts: *ts,
                    open: open_value,
                    high: high_value,
                    low: low_value,
                    close: close_value,
                });
            }

            normalize_flat_opens(symbol, &mut candles);

            if !candles.is_empty() {
                return Ok(candles);
            }

            last_error = "No usable price rows returned".to_string();
        }
    }

    Err(last_error)
}

fn normalize_flat_opens(symbol: &str, candles: &mut [Candle]) {
    // Yahoo occasionally returns BTC rows where open == close for nearly every candle.
    // In that case, use the previous close as open so candle direction is visible.
    if symbol != "BTC-USD" {
        return;
    }

    for i in 1..candles.len() {
        let prev_close = candles[i - 1].close;
        let current = &mut candles[i];
        if (current.open - current.close).abs() < f64::EPSILON {
            current.open = prev_close;
            current.high = current.high.max(current.open).max(current.close);
            current.low = current.low.min(current.open).min(current.close);
        }
    }
}
