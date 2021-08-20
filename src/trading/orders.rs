#![allow(non_camel_case_types)]

use eyre::Result;
use serde::{Deserialize, Serialize};

use crate::{build_request_post, Class, Duration, OrderType, Side, TradierConfig};

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub id: u64,
    pub status: String,
    pub partner_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderResponse {
    pub order: Order,
}

#[derive(Debug, Serialize, Deserialize)]
struct Body {
    class: Class,
    symbol: String,
    side: Side,
    quantity: u64,
    #[serde(alias = "type")]
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
    let response: OrderResponse = request.send()?.json()?;

    Ok(response)
}

#[cfg(test)]
mod tests {
    use mockito::mock;

    use crate::{trading::orders::post_order, Class, Duration, OrderType, Side, TradierConfig};

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
}
