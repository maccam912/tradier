#[cfg(test)]
use mockito;

use once_cell::sync::Lazy;
use reqwest::blocking::RequestBuilder;
use serde::Serialize;

pub mod account;
pub mod market_data;

const VERSION: &str = "v1";

static CLIENT: Lazy<reqwest::blocking::Client> = Lazy::new(reqwest::blocking::Client::new);

static CONFIG: Lazy<config::Config> = Lazy::new(|| {
    config::Config::default()
        .with_merged(config::File::with_name("Config"))
        .unwrap_or_default()
});

static TOKEN: Lazy<String> = Lazy::new(|| CONFIG.get_str("token").unwrap());

#[cfg(test)]
static ENDPOINT: Lazy<String> = Lazy::new(|| mockito::server_url());

#[cfg(not(test))]
static ENDPOINT: Lazy<String> = Lazy::new(|| CONFIG.get_str("endpoint").unwrap());

fn endpoint(path: &str) -> String {
    let endpoint: &str = &ENDPOINT;
    format!("{}/{}/{}", endpoint, VERSION, path)
}

fn build_request_get(
    path: &str,
    _body: Option<impl Serialize>,
    query: Option<impl Serialize>,
) -> RequestBuilder {
    let token: &str = &TOKEN;
    let mut request = CLIENT.get(endpoint(path));
    request = request.header("Accept", "application/json");
    request = request.header("Authorization", format!("Bearer {}", token));
    if let Some(q) = query {
        request = request.query(&q);
    }
    request
}
