pub mod chart;
pub mod holdings;
pub mod options;

pub use chart::{Candle, Range, fetch_candles};
pub use holdings::Holdings;

#[derive(Clone, Copy, Debug)]
pub enum Headers {
    SnP500,
    Tsx,
    Btc,
    CrudeOil,
    CADtoUSD,
}

impl Headers {
    pub const fn all() -> [Headers; 5] {
        [
            Headers::SnP500,
            Headers::Tsx,
            Headers::Btc,
            Headers::CrudeOil,
            Headers::CADtoUSD,
        ]
    }

    pub const fn label(self) -> &'static str {
        match self {
            Headers::SnP500 => "S&P 500",
            Headers::Tsx => "TSX",
            Headers::Btc => "BTC",
            Headers::CrudeOil => "Crude Oil",
            Headers::CADtoUSD => "CAD/USD",
        }
    }

    pub const fn symbol(self) -> &'static str {
        match self {
            Headers::SnP500 => "%5EGSPC",
            Headers::Tsx => "%5EGSPTSE",
            Headers::Btc => "BTC-USD",
            Headers::CrudeOil => "CL=F",
            Headers::CADtoUSD => "CADUSD=X",
        }
    }
}
