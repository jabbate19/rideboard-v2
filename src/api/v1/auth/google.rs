use crate::api::v1::auth::common;
use crate::api::v1::auth::models::UserRealm;
use crate::api::v1::auth::models::{GoogleUserInfo, UserInfo};
use crate::AppState;
use actix_session::Session;
use actix_web::http::header;
use actix_web::{get, web, Scope};
use actix_web::{HttpResponse, Responder};
use oauth2::reqwest::async_http_client;
use oauth2::{AuthorizationCode, TokenResponse};
use reqwest::Client;
use serde::Deserialize;
use utoipa::{OpenApi, ToSchema};

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
        &data.google_oauth,
        Vec::from([
            "openid".to_string(),
            "profile".to_string(),
            "email".to_string(),
        ]),
    )
    .await
}

#[derive(Deserialize, ToSchema)]
pub struct AuthRequest {
    code: String,
    #[allow(dead_code)]
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
    //let _scope = params.scope.clone();

    // Exchange the code with a token.
    let token = &data
        .google_oauth
        .exchange_code(code)
        .request_async(async_http_client)
        .await
        .unwrap();

    let client = Client::new();

    let user_info: GoogleUserInfo = client
        .get(&data.google_userinfo_url)
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    sqlx::query!(
        "INSERT INTO users (id, realm, name, email) VALUES ($1, $2, $3, $4) ON CONFLICT (id) DO UPDATE SET realm = EXCLUDED.realm, name = EXCLUDED.name, email = EXCLUDED.email;",
        user_info.sub,
        UserRealm::Google as _,
        format!("{} {}", user_info.given_name, user_info.family_name),
        user_info.email
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
    web::scope("/google").service(login).service(auth)
}
