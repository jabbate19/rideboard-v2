use actix_session::Session;
use actix_web::http::header;
use actix_web::{web, HttpResponse, Responder};
use oauth2::basic::BasicClient;
use oauth2::{AuthorizationCode, CsrfToken, PkceCodeChallenge, Scope as OAuthScope};

pub async fn login(client: &BasicClient, scopes: Vec<String>) -> impl Responder {
    //let (pkce_code_challenge, _pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the authorization URL to which we'll redirect the user.
    let mut configured_client = client.authorize_url(CsrfToken::new_random);

    for scope in scopes {
        configured_client = configured_client.add_scope(OAuthScope::new(scope));
    }

    let (authorize_url, _csrf_state) = configured_client
        //.set_pkce_challenge(pkce_code_challenge)
        .url();

    HttpResponse::Ok().body(authorize_url.to_string())
}
