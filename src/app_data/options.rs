use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")] 
struct OptionsEnvelope {
    option_chain: Option<OptionChainNode>,
}

#[derive(Deserialize)]
struct OptionChainNode {
    result: Option<Vec<OptionResultNode>>,
    #[allow(dead_code)]
    error: Option<serde_json::Value>
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")] 
struct OptionResultNode {
    expiration_dates: Option<Vec<i64>>,
    quote: Option<OptionQuoteNode>,
    options: Option<Vec<OptionByDateNode>>,
}

#[derive(Deserialize)]
struct OptionQuoteNode {
    currency: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")] 
struct OptionByDateNode {
    expiration_date: Option<i64>,
    calls: Option<Vec<OptionsContractNode>>,
    puts: Option<Vec<OptionsContractNode>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")] 
struct OptionsContractNode {
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

