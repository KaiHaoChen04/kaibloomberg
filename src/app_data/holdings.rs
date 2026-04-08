use std::collections::HashMap;
use crossterm::event::{KeyCode, KeyEvent};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Stock {
    average_price: f64,
    quantity: f64,
}
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(transparent)]
pub struct Holdings {
    pub holding_list: HashMap<String, Stock>,
}
impl Default for Stock {
    fn default() -> Self {
        Self { 
            average_price: 0.0, 
            quantity: 0.0 
        }
    }
}

impl Stock {
    pub fn get_avg_price(&self) -> f64 {
        self.average_price
    }
    pub fn get_quantity(&self) -> f64 {
        self.quantity
    }
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
pub fn add_to_list(hold: &mut Holdings,key: KeyEvent, input_buffer: &mut String) {
    match key.code {
        KeyCode::Char(ch) => {
            input_buffer.push(ch);
        }
        KeyCode::Backspace => {
            input_buffer.pop();
        }
        KeyCode::Enter => {
            let symbol = input_buffer.trim().to_uppercase();
            if !symbol.is_empty() {
                hold.holding_list.entry(symbol).or_insert(Stock::default());
            }
            input_buffer.clear();
        }
        _ => {}
    }
}