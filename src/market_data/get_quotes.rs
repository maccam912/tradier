#![allow(non_camel_case_types)]
use chrono::NaiveDate;
use eyre::Result;
use serde::{Deserialize, Serialize};

use crate::build_request_get;

#[derive(Debug, Serialize, Deserialize)]
pub enum QuoteType {
    stock,
    option,
    etf,
    index,
    mutual_fund,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OptionType {
    put,
    call,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Quote {
    pub symbol: String,
    pub description: String,
    pub exch: String,
    #[serde(alias = "type")]
    pub quote_type: QuoteType,
    pub last: Option<f64>,
    pub change: Option<f64>,
    pub volume: i64,
    pub open: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub close: Option<f64>,
    pub bid: f64,
    pub ask: f64,
    pub underlying: Option<String>,
    pub change_percentage: Option<f64>,
    pub average_volume: i64,
    pub last_volume: i64,
    pub trade_date: i64,
    pub prevclose: Option<f64>,
    pub week_52_high: f64,
    pub week_52_low: f64,
    pub bidsize: i64,
    pub bidexch: String,
    pub bid_date: i64,
    pub asksize: i64,
    pub askexch: String,
    pub ask_date: i64,
    pub open_interest: Option<i64>,
    pub contract_size: Option<i64>,
    pub expiration_date: Option<NaiveDate>,
    pub expiration_type: Option<String>,
    pub option_type: Option<OptionType>,
    pub root_symbols: Option<String>,
    pub root_symbol: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Quotes {
    pub quote: Vec<Quote>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetQuotes {
    pub quotes: Quotes,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Query {
    greeks: bool,
}

pub fn get_quotes(symbols: Vec<String>, greeks: Option<bool>) -> Result<GetQuotes> {
    let query = Query {
        greeks: greeks.unwrap_or(false),
    };

    let request = build_request_get(
        &format!("markets/quotes?{}", symbols.join(",")),
        None::<()>,
        Some(query),
    );
    let response: GetQuotes = request.send()?.json()?;

    Ok(response)
}

#[cfg(test)]
mod tests {
    use mockito::mock;

    use crate::market_data::get_quotes::get_quotes;

    #[test]
    fn test_get_quotes() {
        let _m = mock(
            "GET",
            "/v1/markets/quotes?AAPL,VXX190517P00016000&greeks=false",
        )
        .with_status(200)
        .with_body(include_str!("test_requests/get_quotes.json"))
        .create();

        let response = get_quotes(vec!["AAPL".into(), "VXX190517P00016000".into()], None);
        assert!(response.is_ok());
    }
}
