#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use chrono::{DateTime, Utc};
use eyre::Result;
use serde::{Deserialize, Serialize};

use crate::{build_request_get, Class, Duration, OrderStatus, OrderType, Side, TradierConfig};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Order {
    pub id: u64,
    #[serde(alias = "type")]
    pub order_type: OrderType,
    pub symbol: String,
    pub side: Side,
    pub quantity: f64,
    pub status: OrderStatus,
    pub duration: Duration,
    pub price: Option<f64>,
    pub avg_fill_price: f64,
    pub exec_quantity: f64,
    pub last_fill_price: f64,
    pub last_fill_quantity: f64,
    pub remaining_quantity: f64,
    pub create_date: DateTime<Utc>,
    pub transaction_date: DateTime<Utc>,
    pub class: Class,
    pub leg: Option<Vec<Order>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Orders {
    pub order: Vec<Order>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrdersRoot {
    pub orders: Orders,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NoOrdersRoot {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum MaybeOrdersRoot {
    SomeOrders(OrdersRoot),
    NoneOrders(NoOrdersRoot),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Query {
    includeTags: bool,
}

impl From<MaybeOrdersRoot> for OrdersRoot {
    fn from(o: MaybeOrdersRoot) -> Self {
        match o {
            MaybeOrdersRoot::SomeOrders(or) => or,
            _ => OrdersRoot {
                orders: Orders { order: vec![] },
            },
        }
    }
}

pub fn get_orders(
    config: &TradierConfig,
    account_id: String,
    includeTags: bool,
) -> Result<OrdersRoot> {
    let query = Query { includeTags };

    let response: MaybeOrdersRoot = build_request_get(
        config,
        &format!("accounts/{}/orders", account_id),
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

    use crate::{account::get_orders::get_orders, TradierConfig};

    #[test]
    fn test_get_orders() {
        let _m = mock("GET", "/v1/accounts/VA000000/orders?includeTags=false")
            .with_status(200)
            .with_body(include_str!("test_requests/get_orders.json"))
            .create();

        let config = TradierConfig {
            token: "xxx".into(),
            endpoint: mockito::server_url(),
        };

        let response = get_orders(&config, "VA000000".into(), false);
        assert!(response.is_ok());
    }
}
