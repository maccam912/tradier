#![allow(non_camel_case_types)]

use eyre::Result;
use serde::{Deserialize, Serialize};

use crate::build_request_get;

#[derive(Debug, Serialize, Deserialize)]
enum Type {
    cash,
    margin,
}

#[derive(Debug, Serialize, Deserialize)]
struct Margin {
    fed_call: f64,
    maintenance_call: f64,
    option_buying_power: f64,
    stock_buying_power: f64,
    stock_short_value: f64,
    sweep: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Cash {
    cash_available: f64,
    sweep: f64,
    unsettled_funds: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Pdt {
    fed_call: f64,
    maintenance_call: f64,
    option_buying_power: f64,
    stock_buying_power: f64,
    stock_short_value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Balances {
    option_short_value: f64,
    total_equity: f64,
    account_number: String,
    account_type: Type,
    close_pl: f64,
    current_requirement: f64,
    equity: f64,
    long_market_value: f64,
    market_value: f64,
    open_pl: f64,
    option_long_value: f64,
    option_requirement: f64,
    pending_orders_count: u64,
    short_market_value: f64,
    stock_long_value: f64,
    total_cash: f64,
    uncleared_funds: f64,
    pending_cash: f64,
    margin: Margin,
    cash: Cash,
    pdt: Pdt,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BalancesRoot {
    balances: Balances,
}

pub async fn get_balances(account_id: String) -> Result<BalancesRoot> {
    let response: BalancesRoot = build_request_get(
        &format!("accounts/{}/balances", account_id),
        None::<()>,
        None::<()>,
    )
    .send()
    .await?
    .json()
    .await?;

    Ok(response)
}

#[cfg(test)]
mod tests {
    use mockito::mock;

    use crate::account::get_balances::get_balances;

    #[tokio::test]
    async fn test_get_user_profile() {
        let _m = mock("GET", "/v1/accounts/VA000000/balances")
            .with_status(200)
            .with_body(include_str!("test_requests/get_balances.json"))
            .create();

        get_balances("VA000000".into()).await.unwrap();
        let response = get_balances("VA000000".into()).await;
        assert!(response.is_ok());
    }
}
