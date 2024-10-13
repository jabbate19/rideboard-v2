use crate::api::v1::auth::models::UserRealm;
use crate::api::v1::auth::models::{CSHUserInfo, UserInfo};
use crate::AppState;
use actix_session::Session;
use actix_web::http::header;
use actix_web::{get, Scope};
use actix_web::{web, HttpResponse, Responder};
use oauth2::reqwest::async_http_client;
use oauth2::{AuthorizationCode, TokenResponse};
use reqwest::Client;
use serde::Deserialize;
use utoipa::{OpenApi, ToSchema};

use crate::api::v1::auth::common;

#[derive(OpenApi)]
#[openapi(paths(login, auth,), components(schemas(AuthRequest)))]
pub(super) struct ApiDoc;

#[utoipa::path(
    responses(
        (status = 200, description = "Redirect to OAuth2 Link to Log In")
    )
)]
#[get("/")]
async fn login(data: web::Data<AppState>) -> impl Responder {
    common::login(
        &data.csh_oauth,
        Vec::from(["house-service-oidc".to_string()]),
    )
    .await
}

#[derive(Deserialize, ToSchema)]
pub struct AuthRequest {
    code: String,
    state: String,
}

#[utoipa::path(
    responses(
        (status = 200, description = "Redirect to OAuth2 to verify code and update user info.")
    )
)]
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

    sqlx::query!(
        "INSERT INTO users (id, realm, name) VALUES ($1, $2, $3) ON CONFLICT (id) DO UPDATE SET realm = EXCLUDED.realm, name = EXCLUDED.name;",
        user_info.ldap_id,
        UserRealm::Csh as _,
        format!("{} {}", user_info.given_name, user_info.family_name)
    )
    .execute(&data.db)
    .await.unwrap();

    session.insert("login", true).unwrap();
    session
        .insert("userinfo", UserInfo::from(user_info))
        .unwrap();

    HttpResponse::Found()
        .append_header((header::LOCATION, "/"))
        .finish()
}

pub fn scope() -> Scope {
    web::scope("/csh").service(login).service(auth)
}
