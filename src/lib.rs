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

pub mod account;
pub mod market_data;
pub mod trading;
