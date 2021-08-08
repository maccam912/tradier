#![allow(non_camel_case_types)]

use chrono::{DateTime, Utc};
use eyre::Result;
use serde::{Deserialize, Serialize};

use crate::build_request;

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
struct Journal {
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
struct TradeEvent {
    amount: f64,
    date: DateTime<Utc>,
    #[serde(alias = "type")]
    event_type: EventTypeEnum,
    trade: Trade,
}

#[derive(Debug, Serialize, Deserialize)]
enum EventTypeEnum {
    dividend,
    journal,
    trade,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum EventType {
    Dividend(DividendEvent),
    Journal(JournalEvent),
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

pub async fn get_history(account_id: String) -> Result<HistoryRoot> {
    let response: HistoryEnum = build_request(&format!("accounts/{}/history", account_id))
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

        get_history("VA000000".into()).await.unwrap();
        let response = get_history("VA000000".into()).await;
        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_get_history_single() {
        let _m = mock("GET", "/v1/accounts/VA000000/history")
            .with_status(200)
            .with_body(include_str!("test_requests/get_history_single.json"))
            .create();

        let response = get_history("VA000000".into()).await;
        assert!(response.is_ok());
    }
}
