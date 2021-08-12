#![allow(non_camel_case_types)]

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use chrono_tz::America::New_York;
use eyre::Result;
use serde::{Deserialize, Serialize};

use crate::build_request_get;

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
struct NaiveSeries {
    data: Vec<NaiveData>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Series {
    data: Vec<Data>,
}

impl From<NaiveSeries> for Series {
    fn from(item: NaiveSeries) -> Self {
        let new_data = item.data.into_iter().map(|i| i.into()).collect();
        Series { data: new_data }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NaiveHistorySeries {
    series: NaiveSeries,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistorySeries {
    series: Series,
}

impl From<NaiveHistorySeries> for HistorySeries {
    fn from(item: NaiveHistorySeries) -> Self {
        HistorySeries {
            series: item.series.into(),
        }
    }
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
    start: Option<NaiveDateTime>,
    end: Option<NaiveDateTime>,
    session_filter: Option<SessionFilter>,
}

pub fn get_time_and_sales(
    symbol: String,
    interval: Option<String>,
    start_utc: Option<DateTime<Utc>>,
    end_utc: Option<DateTime<Utc>>,
    session_filter: Option<SessionFilter>,
) -> Result<HistorySeries> {
    let start = start_utc.map(|dt| dt.with_timezone(&New_York).naive_local());
    let end = end_utc.map(|dt| dt.with_timezone(&New_York).naive_local());

    let query = Query {
        symbol,
        interval,
        start,
        end,
        session_filter,
    };

    println!(
        "{:?}",
        build_request_get("markets/timesales", None::<()>, Some(query))
            .send()?
            .text()?
    );

    let response: NaiveHistorySeries =
        build_request_get("markets/timesales", None::<()>, None::<()>)
            .send()?
            .json()?;

    Ok(response.into())
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
        assert!(false);
    }
}
