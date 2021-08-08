#[cfg(test)]
use mockito;

use once_cell::sync::Lazy;
use reqwest::RequestBuilder;

const VERSION: &str = "v1";

static CLIENT: Lazy<reqwest::Client> = Lazy::new(reqwest::Client::new);

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

fn build_request(path: &str) -> RequestBuilder {
    let token: &str = &TOKEN;
    let mut request = CLIENT.get(endpoint(path));
    request = request.header("Accept", "application/json");
    request = request.header("Authorization", format!("Bearer {}", token));
    request
}

pub mod account;
