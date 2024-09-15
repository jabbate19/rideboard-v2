use actix_session::Session;
use actix_web::http::header;
use actix_web::{HttpResponse, Responder};
use oauth2::{AuthorizationCode, CsrfToken, PkceCodeChallenge, Scope as OAuthScope};

use crate::AppState;
use actix_web::{get, web, Scope};
use serde::Deserialize;

#[get("/")]
async fn login(data: web::Data<AppState>) -> impl Responder {
    // Google supports Proof Key for Code Exchange (PKCE - https://oauth.net/2/pkce/).
    // Create a PKCE code verifier and SHA-256 encode it as a code challenge.
    let (pkce_code_challenge, _pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the authorization URL to which we'll redirect the user.
    let (authorize_url, _csrf_state) = &data
        .google_oauth
        .authorize_url(CsrfToken::new_random)
        // This example is requesting access to the "calendar" features and the user's profile.
        .add_scope(OAuthScope::new("openid".to_string()))
        .add_scope(OAuthScope::new("profile".to_string()))
        .add_scope(OAuthScope::new("email".to_string()))
        // .add_scope(Scope::new(
        //     "https://www.googleapis.com/auth/plus.me".to_string(),
        // ))
        .set_pkce_challenge(pkce_code_challenge)
        .url();

    HttpResponse::Found()
        .append_header((header::LOCATION, authorize_url.to_string()))
        .finish()
}

#[derive(Deserialize)]
pub struct AuthRequest {
    code: String,
    state: String,
    scope: String,
}

#[get("/redirect")]
async fn auth(
    session: Session,
    data: web::Data<AppState>,
    params: web::Query<AuthRequest>,
) -> impl Responder {
    let code = AuthorizationCode::new(params.code.clone());
    let state = CsrfToken::new(params.state.clone());
    let _scope = params.scope.clone();

    // Exchange the code with a token.
    let token = &data.google_oauth.exchange_code(code);

    session.insert("login", true).unwrap();

    let html = format!(
        r#"<html>
        <head><title>OAuth2 Test</title></head>
        <body>
            Google returned the following state:
            <pre>{}</pre>
            Google returned the following token:
            <pre>{:?}</pre>
        </body>
    </html>"#,
        state.secret(),
        token
    );
    HttpResponse::Ok().body(html)
}

pub fn scope() -> Scope {
    web::scope("/google").service(login).service(auth)
}
