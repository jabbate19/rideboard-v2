use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CSHUserInfo {
    email: String,
    given_name: String,
    family_name: String,
    preferred_username: String,
    ldap_id: String,
    groups: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GoogleUserInfo {
    email: String,
    given_name: String,
    family_name: String,
    picture: String,
    sub: String,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AuthType {
    CSH(CSHUserInfo),
    GOOGLE(GoogleUserInfo),
}
