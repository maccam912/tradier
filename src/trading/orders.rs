#![allow(non_camel_case_types)]

use eyre::{eyre, Result};
use optimistic_derives::*;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{
    build_request_del, build_request_post, Class, Duration, OrderType, Side, TradierConfig,
};

#[optimistic_no_c]
pub struct Order {
    pub id: u64,
    pub status: String,
    pub partner_id: Option<String>,
}

#[optimistic_no_c]
pub struct OrderResponse {
    pub order: Order,
}

#[optimistic_no_ceho]
struct Body {
    class: Class,
    symbol: String,
    side: Side,
    quantity: u64,
    #[serde(rename(serialize = "type"))]
    order_type: OrderType,
    duration: Duration,
    price: Option<f64>,
    stop: Option<f64>,
    tag: Option<String>,
}

#[allow(clippy::too_many_arguments)]
pub fn post_order(
    config: &TradierConfig,
    account_id: String,
    class: Class,
    symbol: String,
    side: Side,
    quantity: u64,
    order_type: OrderType,
    duration: Duration,
    price: Option<f64>,
    stop: Option<f64>,
    tag: Option<String>,
) -> Result<OrderResponse> {
    let body = Body {
        class,
        symbol,
        side,
        quantity,
        order_type,
        duration,
        price,
        stop,
        tag,
    };

    let request = build_request_post(
        config,
        &format!("accounts/{}/orders", account_id),
        Some(body),
        None::<()>,
    );
    let response = request.send();
    log::debug!("response: {:?}", response);
    let order_response: Result<OrderResponse, reqwest::Error> = response?.json();
    log::debug!("order_response: {:?}", order_response);
    Ok(order_response?)
}

#[optimistic_no_c]
pub struct CancelledResponse {
    order: Order,
}

pub fn cancel_order(
    config: &TradierConfig,
    account_id: String,
    order_id: i64,
) -> Result<CancelledResponse> {
    let request = build_request_del(
        config,
        &format!("accounts/{}/orders/{}", account_id, order_id),
    );
    let response = request.send()?;
    if response.status().clone() == StatusCode::OK {
        let cancel: CancelledResponse = response.json()?;
        Ok(cancel)
    } else {
        Err(eyre!("{:?}", response.text()))
    }
}

#[cfg(test)]
mod tests {
    use mockito::mock;

    use crate::{
        trading::orders::{cancel_order, post_order},
        Class, Duration, OrderType, Side, TradierConfig,
    };

    #[test]
    fn test_post_order() {
        let _m = mock("POST", "/v1/accounts/VA000000/orders")
            .with_status(200)
            .with_body(include_str!("test_requests/post_order.json"))
            .create();

        let config = TradierConfig {
            token: "xxx".into(),
            endpoint: mockito::server_url(),
        };

        let response = post_order(
            &config,
            "VA000000".into(),
            Class::equity,
            "AAPL".into(),
            Side::buy,
            100,
            OrderType::market,
            Duration::gtc,
            None,
            None,
            None,
        );
        assert!(response.is_ok());
    }

    #[test]
    fn test_del_order() {
        let _m = mock("DELETE", "/v1/accounts/VA000000/orders/1")
            .with_status(200)
            .with_body(include_str!("test_requests/del_order.json"))
            .create();

        let config = TradierConfig {
            token: "xxx".into(),
            endpoint: mockito::server_url(),
        };

        let response = cancel_order(&config, "VA000000".into(), 1);
        println!("{:?}", response);

        assert!(response.is_ok());
    }
}
