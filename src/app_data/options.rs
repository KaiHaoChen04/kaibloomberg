#![allow(dead_code)]

use reqwest::header::SET_COOKIE;
use serde::Deserialize;
use url::form_urlencoded;

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

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionByDateNode {
    pub expiration_date: Option<i64>,
    pub calls: Option<Vec<OptionsContractNode>>,
    pub puts: Option<Vec<OptionsContractNode>>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionsContractNode {
    pub contract_symbol: Option<String>,
    pub expiration: Option<i64>,
    pub last_trade_date: Option<i64>,
    pub strike: Option<f64>,
    pub last_price: Option<f64>,
    pub bid: Option<f64>,
    pub ask: Option<f64>,
    pub volume: Option<u64>,
    pub open_interest: Option<u64>,
    pub implied_volatility: Option<f64>,
    pub in_the_money: Option<bool>,
}

pub async fn fetch_options(symbol: String) -> Result<Vec<OptionByDateNode>, YahooErrors> {
    let client = reqwest::Client::new();
    let encoded = encode_symbol(&symbol);
    let mut empty_chain = false;
    let mut last_error: Option<YahooErrors> = None;
    let (cookie, crumb) = fetch_crumb(&client).await?;
    let crumb = encode_query_component(&crumb);

    for host in ["query1.finance.yahoo.com", "query2.finance.yahoo.com"] {
        let url = format!("https://{host}/v7/finance/options/{encoded}?crumb={crumb}");
        let response = client
            .get(&url)
            .header("Cookie", cookie.as_str())
            .header("User-Agent", "Mozilla/5.0")
            .send()
            .await
            .map_err(|e| YahooErrors::Http(e))?;

        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|e| YahooErrors::Http(e))?;

        if !status.is_success() {
            last_error = Some(match status.as_u16() {
                404 => YahooErrors::NotFound { url },
                429 => YahooErrors::RateLimited { url },
                500..=599 => YahooErrors::ServerError {
                    status: status.as_u16(),
                    url,
                },
                _ => YahooErrors::Status {
                    status: status.as_u16(),
                    url,
                },
            });
            continue;
        }

        let envelope: OptionsEnvelope = serde_json::from_str(&body).map_err(YahooErrors::Json)?;

        let Some(chain) = envelope.option_chain else {
            empty_chain = true;
            continue;
        };

        if let Some(error) = chain.error {
            last_error = Some(YahooErrors::Api(format_option_error(&error)));
            continue;
        }

        let Some(result) = chain.result else {
            empty_chain = true;
            continue;
        };

        let Some(first) = result.into_iter().next() else {
            empty_chain = true;
            continue;
        };

        let Some(options) = first.options else {
            empty_chain = true;
            continue;
        };

        let options: Vec<OptionByDateNode> = options
            .into_iter()
            .map(|by_date| OptionByDateNode {
                expiration_date: by_date.expiration_date,
                calls: by_date.calls,
                puts: by_date.puts,
            })
            .collect();

        return Ok(options);
    }

    if empty_chain {
        return Ok(Vec::new());
    }

    if let Some(error) = last_error {
        return Err(error);
    }

    Ok(Vec::new())
}

fn format_option_error(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(text) => text.clone(),
        _ => value.to_string(),
    }
}

async fn fetch_crumb(client: &reqwest::Client) -> Result<(String, String), YahooErrors> {
    let cookie_response = client
        .get("https://fc.yahoo.com")
        .header("User-Agent", "Mozilla/5.0")
        .send()
        .await
        .map_err(|e| YahooErrors::Http(e))?;

    let cookie = extract_cookie(cookie_response.headers())
        .ok_or_else(|| YahooErrors::Auth("Missing Yahoo cookie".to_string()))?;

    let crumb_response = client
        .get("https://query1.finance.yahoo.com/v1/test/getcrumb")
        .header("Cookie", cookie.as_str())
        .header("User-Agent", "Mozilla/5.0")
        .send()
        .await
        .map_err(|e| YahooErrors::Http(e))?;

    if !crumb_response.status().is_success() {
        return Err(YahooErrors::Auth(format!(
            "Crumb request failed: {}",
            crumb_response.status()
        )));
    }

    let crumb = crumb_response
        .text()
        .await
        .map_err(|e| YahooErrors::Http(e))?;

    if crumb.trim().is_empty() {
        return Err(YahooErrors::Auth("Crumb was empty".to_string()));
    }

    Ok((cookie, crumb))
}

fn extract_cookie(headers: &reqwest::header::HeaderMap) -> Option<String> {
    headers
        .get_all(SET_COOKIE)
        .iter()
        .filter_map(|value| value.to_str().ok())
        .find_map(|value| value.split(';').next())
        .map(|value| value.to_string())
}

fn encode_query_component(value: &str) -> String {
    form_urlencoded::byte_serialize(value.as_bytes()).collect()
}
