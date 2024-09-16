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

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo {
    id: String,
    email: String,
    given_name: String,
    family_name: String,
    picture: String,
}

impl From<CSHUserInfo> for UserInfo {
    fn from(user_info: CSHUserInfo) -> Self {
        Self {
            id: user_info.ldap_id,
            email: user_info.email,
            given_name: user_info.given_name,
            family_name: user_info.family_name,
            picture: format!("https://profiles.csh.rit.edu/image/{}", user_info.preferred_username)
        }
    }
}

impl From<GoogleUserInfo> for UserInfo {
    fn from(user_info: GoogleUserInfo) -> Self {
        Self {
            id: user_info.sub,
            email: user_info.email,
            given_name: user_info.given_name,
            family_name: user_info.family_name,
            picture: user_info.picture
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AuthType {
    CSH(CSHUserInfo),
    GOOGLE(GoogleUserInfo),
}

impl From<AuthType> for UserInfo {
    fn from(user_info: AuthType) -> Self {
        match user_info {
            AuthType::CSH(info) => UserInfo::from(info),
            AuthType::GOOGLE(info) => UserInfo::from(info),
        }
    }
}
