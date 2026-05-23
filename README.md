# Kaibloomberg

A Rust terminal UI (TUI) for tracking market headers, portfolio holdings, and options chains in a single console app. It pulls live price data from Yahoo Finance, renders line and candlestick charts, and provides a lightweight portfolio summary with P/L breakdowns.

## Features
- Live price charts for S&P 500, TSX, BTC, crude oil, and CAD/USD.
- Line and candlestick chart modes.
- Portfolio list with quick symbol switching.
- Holdings summary table and pie chart allocation view.
- Options chain viewer with calls/puts toggle and expirations.
- No API keys required (uses Yahoo Finance endpoints).

## Screens
- Main: header tabs, live chart, portfolio list, status panel.
- Portfolio: holdings table with P/L and allocation pie chart.
- Options: chain table by expiration and side (calls/puts).

## Controls

### Global
- `q` quit
- `Tab` toggle between Main and Portfolio screens (Options returns to Main)

### Main screen
- Left/Right: switch header tabs
- `a` add ticker to portfolio list
- `d` remove selected ticker from portfolio list
- `t` toggle active symbol source (header vs portfolio)
- `l` line chart
- `c` candlestick chart
- Up/Down or `k`/`j`: move portfolio selection
- `o` open options screen for active symbol

### Portfolio screen
- `a` add a holding (ticker -> average price -> quantity)
- `t` toggle active symbol source (header vs portfolio)
- `o` open options screen for active symbol
- `d` delete holding (not implemented yet)

### Options screen
- `c` show calls
- `p` show puts
- Left/Right: change expiration
- `r` refresh options chain
- `Tab` return to Main

### Input mode
- `Enter` confirm
- `Esc` cancel

## Data sources
- Prices: Yahoo Finance chart endpoint
- Options: Yahoo Finance options chain endpoint

If you hit rate limits or see empty data, wait a bit and try again.

## Build and Run

```bash
cargo run
```

For a release build:

```bash
cargo build --release
./target/release/kaibloomberg
```

## Project layout
- [src/main.rs](src/main.rs): entry point
- [src/app.rs](src/app.rs): app state, input handling, refresh logic
- [src/ui](src/ui): TUI layout and rendering
- [src/app_data](src/app_data): Yahoo Finance fetchers and data models
- [src/utils](src/utils): helpers and error types

## Notes and limitations
- Portfolio delete in the Portfolio screen is not implemented yet.
- Data depends on Yahoo Finance availability and may be delayed or rate-limited.

## License
See [LICENSE](LICENSE).
