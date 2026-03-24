pub fn sanitize_symbol(symbol: &str) -> String {
    symbol
        .trim()
        .to_ascii_uppercase()
        .chars()
        .filter(|ch| {
            ch.is_ascii_alphanumeric() || *ch == '-' || *ch == '.' || *ch == '=' || *ch == '^'
        })
        .collect()
}

pub fn encode_symbol(symbol: &str) -> String {
    symbol.replace('^', "%5E")
}

pub fn value_or_fallback(series: Option<&Vec<Option<f64>>>, index: usize, fallback: f64) -> f64 {
    series
        .and_then(|values| values.get(index))
        .and_then(|value| *value)
        .unwrap_or(fallback)
}

pub fn status_loading(symbol: &str) -> String {
    format!("Loading live data for {}...", symbol)
}

pub fn status_cached(symbol: &str, count: usize) -> String {
    format!("Showing cached {} ({} candles)", symbol, count)
}

pub fn status_updated(symbol: &str, count: usize) -> String {
    format!("Live data updated for {} ({} candles)", symbol, count)
}

pub fn status_failed(symbol: &str, error: &str) -> String {
    format!("Failed to load {}: {}", symbol, error)
}
