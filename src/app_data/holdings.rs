use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Stock {
    average_price: f64,
    quantity: f64,
    symbol: String,
}
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(transparent)]
pub struct Holdings {
    pub holding_list: HashMap<String, Stock>,
}

impl Stock {
    pub fn stock_value(&self, market_price: f64) -> (f64, f64) {
        let current_holdings = self.average_price * self.quantity;
        let current_pricing = market_price * self.quantity;
        let profit_loss = current_pricing - current_holdings;
        let profit_loss_percentage = if self.average_price > 0.0 {
            (market_price / self.average_price) * 100.0
        }
        else {
            0.0
        };
        (profit_loss, profit_loss_percentage)
    }
}
