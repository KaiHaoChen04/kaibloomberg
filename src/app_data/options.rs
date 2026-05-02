#![allow(dead_code)]

use serde::Deserialize;

use crate::utils::{YahooErrors, encode_symbol};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionsEnvelope {
    option_chain: Option<OptionChainNode>,
}

#[derive(Deserialize)]
pub struct OptionChainNode {
    result: Option<Vec<OptionResultNode>>,
    #[allow(dead_code)]
    error: Option<serde_json::Value>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionResultNode {
    expiration_dates: Option<Vec<i64>>,
    quote: Option<OptionQuoteNode>,
    options: Option<Vec<OptionByDateNode>>,
}

#[derive(Deserialize)]
pub struct OptionQuoteNode {
    currency: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionByDateNode {
    pub expiration_date: Option<i64>,
    pub calls: Option<Vec<OptionsContractNode>>,
    pub puts: Option<Vec<OptionsContractNode>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionsContractNode {
    contract_symbol: Option<String>,
    expiration: Option<i64>,
    last_trade_date: Option<i64>,
    strike: Option<f64>,
    last_price: Option<f64>,
    bid: Option<f64>,
    ask: Option<f64>,
    volume: Option<u64>,
    open_interest: Option<u64>,
    implied_volatility: Option<f64>,
    in_the_money: Option<bool>,
}

pub async fn fetch_options(symbol: String) -> Result<Vec<OptionByDateNode>, YahooErrors> {
    let url = format!("https://query1.finance.yahoo.com/v7/finance/options/{}", encode_symbol(&symbol));

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0")
        .send()
        .await
        .map_err(|e| YahooErrors::Http(e))?;

    let envelope: OptionsEnvelope = response
        .json()
        .await
        .map_err(|e| YahooErrors::Http(e))?;
    
    let result = envelope
        .option_chain
        .ok_or_else(|| YahooErrors::MissingData("no option chain".to_string()))?
        .result
        .ok_or_else(|| YahooErrors::MissingData("no result chain".to_string()))?;

    let first = result 
        .into_iter()
        .next()
        .ok_or_else(|| YahooErrors::MissingData("no option result".to_string()))?;

    let options: Vec<OptionByDateNode> = first
        .options
        .ok_or_else(|| YahooErrors::MissingData("no calls/puts".to_string()))?
        .into_iter()
        .map(|by_date| OptionByDateNode {
            expiration_date: by_date.expiration_date,
            calls: by_date.calls,
            puts: by_date.puts,
        })
        .collect();

    Ok(options)
}
