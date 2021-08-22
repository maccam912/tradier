#![allow(non_camel_case_types)]

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use chrono_tz::America::New_York;
use eyre::{eyre, Result};
use optimistic_derives::*;
use serde::{Deserialize, Serialize};

use crate::{build_request_get, TradierConfig};

#[optimistic_no_ceho]
struct NaiveData {
    time: NaiveDateTime,
    timestamp: i64,
    price: f64,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: i64,
    vwap: f64,
}

#[optimistic_no_ceho]
pub struct Data {
    pub time: DateTime<Utc>,
    pub timestamp: i64,
    pub price: f64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: i64,
    pub vwap: f64,
}

impl From<NaiveData> for Data {
    fn from(item: NaiveData) -> Self {
        let time = New_York.from_local_datetime(&item.time).unwrap();
        Data {
            time: time.with_timezone(&Utc),
            timestamp: item.timestamp,
            price: item.price,
            open: item.open,
            high: item.high,
            low: item.low,
            close: item.close,
            volume: item.volume,
            vwap: item.vwap,
        }
    }
}

#[optimistic_no_ceho]
struct NaiveSeries {
    data: Vec<NaiveData>,
}

#[optimistic_no_ceho]
pub struct Series {
    pub data: Vec<Data>,
}

impl From<NaiveSeries> for Series {
    fn from(item: NaiveSeries) -> Self {
        let new_data = item.data.into_iter().map(|i| i.into()).collect();
        Series { data: new_data }
    }
}

#[optimistic_no_ceho]
pub struct NaiveHistorySeries {
    series: NaiveSeries,
}

#[optimistic_no_ceho]
pub struct HistorySeries {
    pub series: Series,
}

impl From<NaiveHistorySeries> for HistorySeries {
    fn from(item: NaiveHistorySeries) -> Self {
        HistorySeries {
            series: item.series.into(),
        }
    }
}

#[optimistic]
pub enum SessionFilter {
    all,
    open,
}

#[optimistic_no_c]
struct Query {
    symbol: String,
    interval: Option<String>,
    start: Option<String>,
    end: Option<String>,
    session_filter: Option<SessionFilter>,
}

pub fn get_time_and_sales(
    config: &TradierConfig,
    symbol: String,
    interval: Option<String>,
    start_utc: Option<DateTime<Utc>>,
    end_utc: Option<DateTime<Utc>>,
    session_filter: Option<SessionFilter>,
) -> Result<HistorySeries> {
    let start = start_utc.map(|dt| dt.with_timezone(&New_York).naive_local());
    let end = end_utc.map(|dt| dt.with_timezone(&New_York).naive_local());

    let start_str = start.map(|dt| dt.format("%Y-%m-%d %H:%M").to_string());
    let end_str = end.map(|dt| dt.format("%Y-%m-%d %H:%M").to_string());

    let query = Query {
        symbol,
        interval,
        start: start_str,
        end: end_str,
        session_filter,
    };

    let request = build_request_get(config, "markets/timesales", None::<()>, Some(query.clone()));
    log::debug!("Request: {:?}", request);
    let response: Result<NaiveHistorySeries, reqwest::Error> = request.send()?.json();
    log::debug!("Response: {:?}", response);

    match response {
        Ok(resp) => Ok(resp.into()),
        Err(_) => {
            let err = build_request_get(config, "markets/timesales", None::<()>, Some(query))
                .send()?
                .text()?;
            Err(eyre!("{:?}", err))
        }
    }
}

#[cfg(test)]
mod tests {
    use mockito::mock;

    use crate::{market_data::get_time_and_sales::get_time_and_sales, TradierConfig};

    #[test]
    fn test_get_time_and_sales() {
        let _m = mock("GET", "/v1/markets/timesales?symbol=AAPL&interval=1min&start=2021-08-12+20%3A00&end=2021-08-13+20%3A00")
            .with_status(200)
            .with_body(include_str!("test_requests/get_time_and_sales.json"))
            .create();

        let config = TradierConfig {
            token: "xxx".into(),
            endpoint: mockito::server_url(),
        };
        let start = chrono::DateTime::parse_from_str(
            "2021 Aug 13 00:00:00 +0000",
            "%Y %b %d %H:%M:%S%.3f %z",
        )
        .unwrap()
        .with_timezone(&chrono::Utc);
        let end = chrono::DateTime::parse_from_str(
            "2021 Aug 14 00:00:00 +0000",
            "%Y %b %d %H:%M:%S%.3f %z",
        )
        .unwrap()
        .with_timezone(&chrono::Utc);
        let response = get_time_and_sales(
            &config,
            "AAPL".into(),
            Some("1min".into()),
            Some(start),
            Some(end),
            None,
        );
        assert!(response.is_ok());
    }
}
