#![allow(non_camel_case_types)]

use chrono::{DateTime, Utc};
use eyre::Result;
use serde::{Deserialize, Serialize};

use crate::build_request_get;

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    time: DateTime<Utc>,
    timestamp: i64,
    price: f64,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: i64,
    vwap: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Series {
    data: Vec<Data>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistorySeries {
    series: Series,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SessionFilter {
    all,
    open,
}

#[derive(Debug, Serialize, Deserialize)]
struct Query {
    symbol: String,
    interval: Option<String>,
    start: Option<DateTime<Utc>>,
    end: Option<DateTime<Utc>>,
    session_filter: Option<SessionFilter>,
}

pub fn get_time_and_sales(
    symbol: String,
    interval: Option<String>,
    start: Option<DateTime<Utc>>,
    end: Option<DateTime<Utc>>,
    session_filter: Option<SessionFilter>,
) -> Result<HistorySeries> {
    let query = Query {
        symbol,
        interval,
        start,
        end,
        session_filter,
    };

    let response = build_request_get("markets/timesales", None::<()>, Some(query))
        .send()?
        .text()?;

    println!("{:?}", response);
    let response: HistorySeries = build_request_get("markets/timesales", None::<()>, None::<()>)
        .send()?
        .json()?;

    Ok(response)
}

#[cfg(test)]
mod tests {
    use mockito::mock;

    use crate::market_data::get_time_and_sales::get_time_and_sales;

    #[test]
    fn test_get_time_and_sales() {
        let _m = mock("GET", "/v1/markets/timesales")
            .with_status(200)
            .with_body(include_str!("test_requests/get_time_and_sales.json"))
            .create();

        get_time_and_sales("AAPL".into(), Some("1min".into()), None, None, None).unwrap();
        let response = get_time_and_sales("AAPL".into(), Some("1min".into()), None, None, None);
        assert!(response.is_ok());
    }
}
