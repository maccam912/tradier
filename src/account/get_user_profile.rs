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
struct Profile {
    id: String,
    name: String,
    account: Vec<Account>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SingleProfile {
    id: String,
    name: String,
    account: Account,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfile {
    profile: Profile,
}

#[derive(Debug, Serialize, Deserialize)]
struct SingleUserProfile {
    profile: SingleProfile,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum ProfileEnum {
    ProfileUnit(SingleUserProfile),
    ProfileVec(UserProfile),
}

impl From<ProfileEnum> for UserProfile {
    fn from(item: ProfileEnum) -> UserProfile {
        match item {
            ProfileEnum::ProfileUnit(unit) => UserProfile {
                profile: Profile {
                    id: unit.profile.id,
                    name: unit.profile.name,
                    account: vec![unit.profile.account],
                },
            },
            ProfileEnum::ProfileVec(profile) => profile,
        }
    }
}

pub async fn get_user_profile() -> Result<UserProfile> {
    let response: ProfileEnum = build_request("user/profile").send().await?.json().await?;

    Ok(response.into())
}

#[cfg(test)]
mod tests {
    use crate::account::get_user_profile::get_user_profile;
    use mockito::mock;

    #[tokio::test]
    async fn test_get_user_profile() {
        let _m = mock("GET", "/v1/user/profile")
            .with_status(200)
            .with_body(include_str!("get_user_profile.json"))
            .create();

        let response = get_user_profile().await;
        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_get_user_profile_single() {
        let _m = mock("GET", "/v1/user/profile")
            .with_status(200)
            .with_body(include_str!("get_user_profile_single.json"))
            .create();

        let response = get_user_profile().await;
        assert!(response.is_ok());
    }
}
