#![allow(non_camel_case_types)]

use chrono::{DateTime, Utc};
use eyre::Result;
use serde::{Deserialize, Serialize};

use crate::build_request;

#[derive(Debug, Serialize, Deserialize)]
struct Position {
    cost_basis: f64,
    date_acquired: DateTime<Utc>,
    id: u64,
    quantity: f64,
    symbol: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Positions {
    position: Vec<Position>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SinglePosition {
    position: Position,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositionsRoot {
    positions: Positions,
}

#[derive(Debug, Serialize, Deserialize)]
struct SinglePositionsRoot {
    positions: SinglePosition,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum PositionsEnum {
    PositionsUnit(SinglePositionsRoot),
    PositionsVec(PositionsRoot),
}

impl From<PositionsEnum> for PositionsRoot {
    fn from(item: PositionsEnum) -> PositionsRoot {
        match item {
            PositionsEnum::PositionsUnit(unit) => PositionsRoot {
                positions: Positions {
                    position: vec![unit.positions.position],
                },
            },
            PositionsEnum::PositionsVec(profile) => profile,
        }
    }
}

pub async fn get_positions(account_id: String) -> Result<PositionsRoot> {
    let response: PositionsEnum = build_request(&format!("accounts/{}/positions", account_id))
        .send()
        .await?
        .json()
        .await?;

    Ok(response.into())
}

#[cfg(test)]
mod tests {
    use mockito::mock;

    use crate::account::get_positions::get_positions;

    #[tokio::test]
    async fn test_get_user_profile() {
        let _m = mock("GET", "/v1/accounts/VA000000/positions")
            .with_status(200)
            .with_body(include_str!("test_requests/get_positions.json"))
            .create();

        let response = get_positions("VA000000".into()).await;
        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_get_user_profile_single() {
        let _m = mock("GET", "/v1/accounts/VA000000/positions")
            .with_status(200)
            .with_body(include_str!("test_requests/get_positions_single.json"))
            .create();

        get_positions("VA000000".into()).await.unwrap();
        let response = get_positions("VA000000".into()).await;
        assert!(response.is_ok());
    }
}
