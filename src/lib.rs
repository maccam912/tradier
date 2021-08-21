#![allow(non_camel_case_types)]

use once_cell::sync::Lazy;
use reqwest::blocking::RequestBuilder;
use serde::{Deserialize, Serialize};

const VERSION: &str = "v1";

static CLIENT: Lazy<reqwest::blocking::Client> = Lazy::new(reqwest::blocking::Client::new);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradierConfig {
    pub token: String,
    pub endpoint: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OrderType {
    market,
    limit,
    stop,
    stop_limit,
    debit,
    credit,
    even,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Class {
    equity,
    option,
    multileg,
    combo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Side {
    buy,
    buy_to_cover,
    sell,
    sell_short,
    buy_to_open,
    buy_to_close,
    sell_to_open,
    sell_to_close,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Duration {
    day,
    gtc,
    pre,
    post,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OrderStatus {
    open,
    partially_filled,
    filled,
    expired,
    canceled,
    pending,
    rejected,
    calculated,
    accepted_for_bidding,
    error,
    held,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Classification {
    individual,
    entity,
    joint_survivor,
    traditional_ira,
    roth_ira,
    rollover_ira,
    sep_ira,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AccountType {
    cash,
    margin,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AccountStatus {
    active,
    closed,
}

fn endpoint(config: &TradierConfig, path: &str) -> String {
    let endpoint: &str = &config.endpoint;
    format!("{}/{}/{}", endpoint, VERSION, path)
}

fn build_request_get(
    config: &TradierConfig,
    path: &str,
    _body: Option<impl Serialize>,
    query: Option<impl Serialize>,
) -> RequestBuilder {
    let token: &str = &config.token;
    let mut request = CLIENT.get(endpoint(config, path));
    request = request.header("Accept", "application/json");
    request = request.header("Authorization", format!("Bearer {}", token));
    if let Some(q) = query {
        request = request.query(&q);
    }
    request
}

fn build_request_post(
    config: &TradierConfig,
    path: &str,
    body: Option<impl Serialize>,
    _query: Option<impl Serialize>,
) -> RequestBuilder {
    let token: &str = &config.token;
    let mut request = CLIENT.post(endpoint(config, path));
    request = request.header("Accept", "application/json");
    request = request.header("Authorization", format!("Bearer {}", token));
    if let Some(b) = body {
        request = request.form(&b);
    }
    request
}

fn build_request_del(config: &TradierConfig, path: &str) -> RequestBuilder {
    let token: &str = &config.token;
    let mut request = CLIENT.delete(endpoint(config, path));
    request = request.header("Accept", "application/json");
    request = request.header("Authorization", format!("Bearer {}", token));
    request
}

pub mod account;
pub mod market_data;
pub mod trading;
