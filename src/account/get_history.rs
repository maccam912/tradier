#![allow(non_camel_case_types, clippy::upper_case_acronyms)]

use chrono::{DateTime, Utc};
use eyre::Result;
use serde::{Deserialize, Serialize};

use crate::build_request_get;

#[derive(Debug, Serialize, Deserialize)]
enum TradeType {
    Equity,
    Option,
}

#[derive(Debug, Serialize, Deserialize)]
struct Dividend {
    description: String,
    quantity: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct DivAdj {
    description: String,
    quantity: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Trade {
    commission: f64,
    description: String,
    price: f64,
    quantity: f64,
    symbol: String,
    trade_type: TradeType,
}

#[derive(Debug, Serialize, Deserialize)]
struct DividendEvent {
    amount: f64,
    date: DateTime<Utc>,
    #[serde(alias = "type")]
    event_type: EventTypeEnum,
    adjustment: Dividend,
}

#[derive(Debug, Serialize, Deserialize)]
struct DivAdjEvent {
    amount: f64,
    date: DateTime<Utc>,
    #[serde(alias = "type")]
    event_type: EventTypeEnum,
    adjustment: DivAdj,
}

#[derive(Debug, Serialize, Deserialize)]
struct Journal {
    description: String,
    quantity: f64,
}

#[derive(Debug, Serialize, Deserialize)]
enum OptionType {
    OPTEXP,
    expiration,
}

#[derive(Debug, Serialize, Deserialize)]
struct TradierOption {
    option_type: OptionType,
    description: String,
    quantity: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct JournalEvent {
    amount: f64,
    date: DateTime<Utc>,
    #[serde(alias = "type")]
    event_type: EventTypeEnum,
    journal: Journal,
}

#[derive(Debug, Serialize, Deserialize)]
struct OptionEvent {
    amount: f64,
    date: DateTime<Utc>,
    #[serde(alias = "type")]
    event_type: EventTypeEnum,
    option: TradierOption,
}

#[derive(Debug, Serialize, Deserialize)]
struct TradeEvent {
    amount: f64,
    date: DateTime<Utc>,
    #[serde(alias = "type")]
    event_type: EventTypeEnum,
    trade: Trade,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EventTypeEnum {
    trade,
    option,
    ach,
    wire,
    dividend,
    fee,
    tax,
    journal,
    check,
    transfer,
    adjustment,
    interest,
    DIVADJ,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum EventType {
    Dividend(DividendEvent),
    DivAdj(DivAdjEvent),
    Journal(JournalEvent),
    TradierOption(OptionEvent),
    Trade(TradeEvent),
}

#[derive(Debug, Serialize, Deserialize)]
struct SingleHistory {
    event: EventType,
}

#[derive(Debug, Serialize, Deserialize)]
struct History {
    event: Vec<EventType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryRoot {
    history: History,
}

#[derive(Debug, Serialize, Deserialize)]
struct SingleHistoryRoot {
    history: SingleHistory,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum HistoryEnum {
    HistoryUnit(SingleHistoryRoot),
    HistoryVec(HistoryRoot),
}

impl From<HistoryEnum> for HistoryRoot {
    fn from(item: HistoryEnum) -> HistoryRoot {
        match item {
            HistoryEnum::HistoryUnit(unit) => HistoryRoot {
                history: History {
                    event: vec![unit.history.event],
                },
            },
            HistoryEnum::HistoryVec(root) => root,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Query {
    page: Option<u64>,
    limit: Option<u64>,
    activity_type: Option<EventTypeEnum>,
    start: Option<DateTime<Utc>>,
    end: Option<DateTime<Utc>>,
    symbol: Option<String>,
}

pub async fn get_history(
    account_id: String,
    page: Option<u64>,
    limit: Option<u64>,
    activity_type: Option<EventTypeEnum>,
    start: Option<DateTime<Utc>>,
    end: Option<DateTime<Utc>>,
    symbol: Option<String>,
) -> Result<HistoryRoot> {
    let query = Query {
        page,
        limit,
        activity_type,
        start,
        end,
        symbol,
    };

    let response: HistoryEnum = build_request_get(
        &format!("accounts/{}/history", account_id),
        None::<()>,
        Some(query),
    )
    .send()
    .await?
    .json()
    .await?;

    Ok(response.into())
}

#[cfg(test)]
mod tests {
    use mockito::mock;

    use crate::account::get_history::get_history;

    #[tokio::test]
    async fn test_get_history() {
        let _m = mock("GET", "/v1/accounts/VA000000/history")
            .with_status(200)
            .with_body(include_str!("test_requests/get_history.json"))
            .create();

        let response = get_history("VA000000".into(), None, None, None, None, None, None).await;
        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_get_history_single() {
        let _m = mock("GET", "/v1/accounts/VA000000/history")
            .with_status(200)
            .with_body(include_str!("test_requests/get_history_single.json"))
            .create();

        let response = get_history("VA000000".into(), None, None, None, None, None, None).await;
        assert!(response.is_ok());
    }
}
