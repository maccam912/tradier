#![allow(non_camel_case_types)]

use eyre::Result;
use serde::{Deserialize, Serialize};

use crate::{build_request_get, TradierConfig};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Type {
    cash,
    margin,
    pdt,
}

impl Default for Type {
    fn default() -> Self {
        Type::cash
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Margin {
    pub fed_call: f64,
    pub maintenance_call: f64,
    pub option_buying_power: f64,
    pub stock_buying_power: f64,
    pub stock_short_value: f64,
    pub sweep: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Cash {
    pub cash_available: f64,
    pub sweep: f64,
    pub unsettled_funds: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Pdt {
    pub fed_call: f64,
    pub maintenance_call: f64,
    pub option_buying_power: f64,
    pub stock_buying_power: f64,
    pub stock_short_value: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Balances {
    pub option_short_value: f64,
    pub total_equity: f64,
    pub account_number: String,
    pub account_type: Type,
    pub close_pl: f64,
    pub current_requirement: f64,
    pub equity: f64,
    pub long_market_value: f64,
    pub market_value: f64,
    pub open_pl: f64,
    pub option_long_value: f64,
    pub option_requirement: f64,
    pub pending_orders_count: u64,
    pub short_market_value: f64,
    pub stock_long_value: f64,
    pub total_cash: f64,
    pub uncleared_funds: f64,
    pub pending_cash: f64,
    pub margin: Option<Margin>,
    pub cash: Option<Cash>,
    pub pdt: Option<Pdt>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BalancesRoot {
    pub balances: Balances,
}

pub fn get_balances(config: &TradierConfig, account_id: String) -> Result<BalancesRoot> {
    let response: BalancesRoot = build_request_get(
        config,
        &format!("accounts/{}/balances", account_id),
        None::<()>,
        None::<()>,
    )
    .send()?
    .json()?;

    Ok(response)
}

#[cfg(test)]
mod tests {
    use mockito::mock;

    use crate::{account::get_balances::get_balances, TradierConfig};

    #[test]
    fn test_get_user_profile() {
        let _m = mock("GET", "/v1/accounts/VA000000/balances")
            .with_status(200)
            .with_body(include_str!("test_requests/get_balances.json"))
            .create();

        let config = TradierConfig {
            token: "xxx".into(),
            endpoint: mockito::server_url(),
        };

        get_balances(&config, "VA000000".into()).unwrap();
        let response = get_balances(&config, "VA000000".into());
        assert!(response.is_ok());
    }
}
