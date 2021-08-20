#![allow(non_camel_case_types)]

use chrono::{DateTime, Utc};
use eyre::Result;
use serde::{Deserialize, Serialize};

use crate::{build_request_get, AccountStatus, AccountType, Classification, TradierConfig};

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub account_number: String,
    pub classification: Classification,
    pub date_created: DateTime<Utc>,
    pub day_trader: bool,
    pub option_level: u8,
    pub status: AccountStatus,
    #[serde(alias = "type")]
    pub account_type: AccountType,
    pub last_update_date: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub account: Vec<Account>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SingleProfile {
    id: String,
    name: String,
    account: Account,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfile {
    pub profile: Profile,
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
            ProfileEnum::ProfileVec(root) => root,
        }
    }
}

pub fn get_user_profile(config: &TradierConfig) -> Result<UserProfile> {
    let response: ProfileEnum = build_request_get(config, "user/profile", None::<()>, None::<()>)
        .send()?
        .json()?;

    Ok(response.into())
}

#[cfg(test)]
mod tests {
    use mockito::mock;

    use crate::{account::get_user_profile::get_user_profile, TradierConfig};

    #[test]
    fn test_get_user_profile() {
        let _m = mock("GET", "/v1/user/profile")
            .with_status(200)
            .with_body(include_str!("test_requests/get_user_profile.json"))
            .create();

        let config = TradierConfig {
            token: "xxx".into(),
            endpoint: mockito::server_url(),
        };

        let response = get_user_profile(&config);
        assert!(response.is_ok());
    }

    #[test]
    fn test_get_user_profile_single() {
        let _m = mock("GET", "/v1/user/profile")
            .with_status(200)
            .with_body(include_str!("test_requests/get_user_profile_single.json"))
            .create();

        let config = TradierConfig {
            token: "xxx".into(),
            endpoint: mockito::server_url(),
        };

        let response = get_user_profile(&config);
        assert!(response.is_ok());
    }
}
