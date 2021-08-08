#![allow(non_camel_case_types)]

use chrono::{DateTime, Utc};
use eyre::Result;
use serde::{Deserialize, Serialize};

use crate::build_request;

#[derive(Debug, Serialize, Deserialize)]
enum Classification {
    individual,
    entity,
    joint_survivor,
    traditional_ira,
    roth_ira,
    rollover_ira,
    sep_ira,
}

#[derive(Debug, Serialize, Deserialize)]
enum Status {
    active,
    closed,
}

#[derive(Debug, Serialize, Deserialize)]
enum Type {
    cash,
    margin,
}

#[derive(Debug, Serialize, Deserialize)]
struct Account {
    account_number: String,
    classification: Classification,
    date_created: DateTime<Utc>,
    day_trader: bool,
    option_level: u8,
    status: Status,
    #[serde(alias = "type")]
    account_type: Type,
    last_update_date: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum AccountEnum {
    Vec(Account),
    Account,
}

#[derive(Debug, Serialize, Deserialize)]
struct Profile {
    id: String,
    name: String,
    account: AccountEnum,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfile {
    profile: Profile,
}

pub async fn get_user_profile() -> Result<UserProfile> {
    println!(
        "{:?}",
        build_request("user/profile").send().await?.text().await?
    );
    let response: UserProfile = build_request("user/profile").send().await?.json().await?;

    Ok(response)
}

#[cfg(test)]
mod tests {
    use crate::account::get_user_profile::get_user_profile;

    #[tokio::test]
    async fn test_get_user_profile() {
        let response = get_user_profile().await;
        assert!(response.is_ok());
    }
}
