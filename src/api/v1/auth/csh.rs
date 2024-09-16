use actix_session::Session;
use actix_web::http::header;
use actix_web::{web, HttpResponse, Responder};
use oauth2::reqwest::{async_http_client, http_client};
use oauth2::{AuthorizationCode, CsrfToken, PkceCodeChallenge, TokenResponse};
use reqwest::Client;
use serde_json::Value;

use crate::api::v1::auth::models::{AuthType, CSHUserInfo};
use crate::AppState;
use actix_web::{get, Scope};
use serde::Deserialize;

use crate::api::v1::auth::common;

#[get("/")]
async fn login(data: web::Data<AppState>) -> impl Responder {
    common::login(&data.csh_oauth, Vec::from(["house-service-oidc".to_string()])).await
}

#[derive(Deserialize)]
pub struct AuthRequest {
    code: String,
    state: String,
}

#[get("/redirect")]
async fn auth(
    session: Session,
    data: web::Data<AppState>,
    params: web::Query<AuthRequest>,
) -> impl Responder {
    let code = AuthorizationCode::new(params.code.clone());

    let token = &data
        .csh_oauth
        .exchange_code(code)
        .request_async(async_http_client)
        .await
        .unwrap();

    let client = Client::new();

    let user_info: CSHUserInfo = client
        .get(&data.csh_userinfo_url)
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    session.insert("login", true).unwrap();
    session.insert("userinfo", AuthType::CSH(user_info)).unwrap();

    HttpResponse::Found().append_header((header::LOCATION, "/")).finish()
}

pub fn scope() -> Scope {
    web::scope("/csh").service(login).service(auth)
}
