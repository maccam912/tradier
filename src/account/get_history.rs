#![allow(non_camel_case_types, clippy::upper_case_acronyms)]

use chrono::{DateTime, Utc};
use eyre::Result;
use optimistic_derives::*;
use serde::{Deserialize, Serialize};

use crate::{build_request_get, TradierConfig};

#[optimistic]
enum TradeType {
    Equity,
    Option,
}

#[optimistic_no_ceho]
struct Dividend {
    description: String,
    quantity: f64,
}

#[optimistic_no_ceho]
struct DivAdj {
    description: String,
    quantity: f64,
}

#[optimistic_no_ceho]
struct Trade {
    commission: f64,
    description: String,
    price: f64,
    quantity: f64,
    symbol: String,
    trade_type: TradeType,
}

#[optimistic_no_ceho]
struct DividendEvent {
    amount: f64,
    date: DateTime<Utc>,
    #[serde(alias = "type")]
    event_type: EventTypeEnum,
    adjustment: Dividend,
}

#[optimistic_no_ceho]
struct DivAdjEvent {
    amount: f64,
    date: DateTime<Utc>,
    #[serde(alias = "type")]
    event_type: EventTypeEnum,
    adjustment: DivAdj,
}

#[optimistic_no_ceho]
struct Journal {
    description: String,
    quantity: f64,
}

#[optimistic]
enum OptionType {
    OPTEXP,
    expiration,
}

#[optimistic_no_ceho]
struct TradierOption {
    option_type: OptionType,
    description: String,
    quantity: f64,
}

#[optimistic_no_ceho]
struct JournalEvent {
    amount: f64,
    date: DateTime<Utc>,
    #[serde(alias = "type")]
    event_type: EventTypeEnum,
    journal: Journal,
}

#[optimistic_no_ceho]
struct OptionEvent {
    amount: f64,
    date: DateTime<Utc>,
    #[serde(alias = "type")]
    event_type: EventTypeEnum,
    option: TradierOption,
}

#[optimistic_no_ceho]
struct TradeEvent {
    amount: f64,
    date: DateTime<Utc>,
    #[serde(alias = "type")]
    event_type: EventTypeEnum,
    trade: Trade,
}

#[optimistic]
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

#[optimistic_no_ceho]
#[serde(untagged)]
enum EventType {
    Dividend(DividendEvent),
    DivAdj(DivAdjEvent),
    Journal(JournalEvent),
    TradierOption(OptionEvent),
    Trade(TradeEvent),
}

#[optimistic_no_ceho]
struct SingleHistory {
    event: EventType,
}

#[optimistic_no_ceho]
struct History {
    event: Vec<EventType>,
}

#[optimistic_no_ceho]
pub struct HistoryRoot {
    history: History,
}

#[optimistic_no_ceho]
struct SingleHistoryRoot {
    history: SingleHistory,
}

#[optimistic_no_ceho]
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

#[optimistic_no_c]
struct Query {
    page: Option<u64>,
    limit: Option<u64>,
    activity_type: Option<EventTypeEnum>,
    start: Option<DateTime<Utc>>,
    end: Option<DateTime<Utc>>,
    symbol: Option<String>,
}

#[allow(clippy::too_many_arguments)]
pub fn get_history(
    config: &TradierConfig,
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
        config,
        &format!("accounts/{}/history", account_id),
        None::<()>,
        Some(query),
    )
    .send()?
    .json()?;

    Ok(response.into())
}

#[cfg(test)]
mod tests {
    use mockito::mock;

    use crate::{account::get_history::get_history, TradierConfig};

    #[test]
    fn test_get_history() {
        let _m = mock("GET", "/v1/accounts/VA000000/history")
            .with_status(200)
            .with_body(include_str!("test_requests/get_history.json"))
            .create();

        let config = TradierConfig {
            token: "xxx".into(),
            endpoint: mockito::server_url(),
        };

        let response = get_history(
            &config,
            "VA000000".into(),
            None,
            None,
            None,
            None,
            None,
            None,
        );
        assert!(response.is_ok());
    }

    #[test]
    fn test_get_history_single() {
        let _m = mock("GET", "/v1/accounts/VA000000/history")
            .with_status(200)
            .with_body(include_str!("test_requests/get_history_single.json"))
            .create();

        let config = TradierConfig {
            token: "xxx".into(),
            endpoint: mockito::server_url(),
        };

        let response = get_history(
            &config,
            "VA000000".into(),
            None,
            None,
            None,
            None,
            None,
            None,
        );
        assert!(response.is_ok());
    }
}
