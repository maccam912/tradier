#![allow(non_camel_case_types)]
use chrono::NaiveDate;
use eyre::Result;
use serde::{Deserialize, Serialize};

use crate::build_request_get;

#[derive(Debug, Serialize, Deserialize)]
enum QuoteType {
    stock,
    option,
    etf,
    index,
    mutual_fund,
}

#[derive(Debug, Serialize, Deserialize)]
enum OptionType {
    put,
    call,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Quote {
    symbol: String,
    description: String,
    exch: String,
    #[serde(alias = "type")]
    quote_type: QuoteType,
    last: Option<f64>,
    change: Option<f64>,
    volume: i64,
    open: Option<f64>,
    high: Option<f64>,
    low: Option<f64>,
    close: Option<f64>,
    bid: f64,
    ask: f64,
    underlying: Option<String>,
    change_percentage: Option<f64>,
    average_volume: i64,
    last_volume: i64,
    trade_date: i64,
    prevclose: Option<f64>,
    week_52_high: f64,
    week_52_low: f64,
    bidsize: i64,
    bidexch: String,
    bid_date: i64,
    asksize: i64,
    askexch: String,
    ask_date: i64,
    open_interest: Option<i64>,
    contract_size: Option<i64>,
    expiration_date: Option<NaiveDate>,
    expiration_type: Option<String>,
    option_type: Option<OptionType>,
    root_symbols: Option<String>,
    root_symbol: Option<String>,
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
