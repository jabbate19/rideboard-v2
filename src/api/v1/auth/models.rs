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
    pub id: String,
    pub username: Option<String>,
    pub email: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
    pub groups: Vec<String>
}

impl From<CSHUserInfo> for UserInfo {
    fn from(user_info: CSHUserInfo) -> Self {
        let username = user_info.preferred_username;
        Self {
            id: user_info.ldap_id,
            username: Some(username.clone()),
            email: user_info.email,
            given_name: user_info.given_name,
            family_name: user_info.family_name,
            picture: format!("https://profiles.csh.rit.edu/image/{}", username),
            groups: user_info.groups
        }
    }
}

impl From<GoogleUserInfo> for UserInfo {
    fn from(user_info: GoogleUserInfo) -> Self {
        Self {
            id: user_info.sub,
            username: None,
            email: user_info.email,
            given_name: user_info.given_name,
            family_name: user_info.family_name,
            picture: user_info.picture,
            groups: Vec::new()
        }
    }
}
