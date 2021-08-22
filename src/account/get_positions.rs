#![allow(non_camel_case_types)]

use chrono::{DateTime, Utc};
use eyre::Result;
use optimistic_derives::*;
use serde::{Deserialize, Serialize};

use crate::{build_request_get, TradierConfig};

#[optimistic_no_ceho]
pub struct Position {
    pub cost_basis: f64,
    pub date_acquired: DateTime<Utc>,
    pub id: u64,
    pub quantity: f64,
    pub symbol: String,
}

#[optimistic_no_ceho]
pub struct Positions {
    pub position: Vec<Position>,
}

#[optimistic_no_ceho]
struct SinglePosition {
    position: Position,
}

#[optimistic_no_ceho]
pub struct PositionsRoot {
    pub positions: Positions,
}

#[optimistic_no_ceho]
struct SinglePositionsRoot {
    positions: Option<SinglePosition>,
}

#[optimistic_no_ceho]
struct EmptyPositionsRoot {}

#[optimistic_no_ceho]
#[serde(untagged)]
enum PositionsEnum {
    Unit(SinglePositionsRoot),
    Vec(PositionsRoot),
    Empty(EmptyPositionsRoot),
}

impl From<PositionsEnum> for PositionsRoot {
    fn from(item: PositionsEnum) -> PositionsRoot {
        match item {
            PositionsEnum::Unit(unit) => PositionsRoot {
                positions: Positions {
                    position: match unit.positions {
                        Some(pos) => vec![pos.position],
                        None => vec![],
                    },
                },
            },
            PositionsEnum::Empty(_) => PositionsRoot {
                positions: Positions { position: vec![] },
            },
            PositionsEnum::Vec(root) => root,
        }
    }
}

pub fn get_positions(config: &TradierConfig, account_id: String) -> Result<PositionsRoot> {
    let response: PositionsEnum = build_request_get(
        config,
        &format!("accounts/{}/positions", account_id),
        None::<()>,
        None::<()>,
    )
    .send()?
    .json()?;

    Ok(response.into())
}

#[cfg(test)]
mod tests {
    use mockito::mock;

    use crate::{account::get_positions::get_positions, TradierConfig};

    #[test]
    fn test_get_positions() {
        let _m = mock("GET", "/v1/accounts/VA000000/positions")
            .with_status(200)
            .with_body(include_str!("test_requests/get_positions.json"))
            .create();

        let config = TradierConfig {
            token: "xxx".into(),
            endpoint: mockito::server_url(),
        };

        let response = get_positions(&config, "VA000000".into());
        assert!(response.is_ok());
    }

    #[test]
    fn test_get_positions_single() {
        let _m = mock("GET", "/v1/accounts/VA000000/positions")
            .with_status(200)
            .with_body(include_str!("test_requests/get_positions_single.json"))
            .create();

        let config = TradierConfig {
            token: "xxx".into(),
            endpoint: mockito::server_url(),
        };

        get_positions(&config, "VA000000".into()).unwrap();
        let response = get_positions(&config, "VA000000".into());
        assert!(response.is_ok());
    }
}
