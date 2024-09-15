use std::{
    env,
    future::{ready, Ready},
    task::Poll,
};

use actix_session::SessionExt;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};

pub struct SessionAuth;

impl<S> Transform<S, ServiceRequest> for SessionAuth
where
    S: Service<
        ServiceRequest,
        Response = ServiceResponse<actix_web::body::BoxBody>,
        Error = actix_web::Error,
    >,
    S::Future: 'static,
{
    type Response = ServiceResponse<actix_web::body::BoxBody>;
    type Error = actix_web::Error;
    type InitError = ();
    type Transform = SessionAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SessionAuthMiddleware { service }))
    }
}

pub struct SessionAuthMiddleware<S> {
    service: S,
}

impl<S> Service<ServiceRequest> for SessionAuthMiddleware<S>
where
    S: Service<
        ServiceRequest,
        Response = ServiceResponse<actix_web::body::BoxBody>,
        Error = actix_web::Error,
    >,
    S::Future: 'static,
{
    type Response = ServiceResponse<actix_web::body::BoxBody>;
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, actix_web::Error>>;

    fn poll_ready(&self, ctx: &mut core::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let session = req.get_session();
        if let Ok(Some(true)) = session.get::<bool>("login") {
            // Proceed with the request if the session is valid
            let future = self.service.call(req);

            return Box::pin(async move {
                let response = future.await?;
                Ok(response)
            });
        }
        Box::pin(async { Ok(req.into_response(HttpResponse::Unauthorized().finish())) })
    }
}

fn get_oauth_client(
    client_id: String,
    client_secret: Option<String>,
    auth_url: String,
    token_url: Option<String>,
    redirect_url: String,
) -> BasicClient {
    let client_id = ClientId::new(client_id);
    let client_secret = client_secret.map(ClientSecret::new);
    let auth_url = AuthUrl::new(auth_url).expect("Invalid authorization endpoint URL");

    let token_url = token_url.map(|val| TokenUrl::new(val).expect("Invalid token endpoint URL"));

    BasicClient::new(client_id, client_secret, auth_url, token_url)
        .set_redirect_uri(RedirectUrl::new(redirect_url).expect("Invalid redirect URL"))
}

pub fn get_clients(host: &str, port: i32) -> (BasicClient, BasicClient) {
    let redirect_domain =
        env::var("REDIRECT_DOMAIN").unwrap_or(format!("http://{}:{}", host, port));

    (
        get_oauth_client(
            env::var("GOOGLE_CLIENT_ID")
                .expect("Missing the GOOGLE_CLIENT_ID environment variable."),
            Some(
                env::var("GOOGLE_CLIENT_SECRET")
                    .expect("Missing the GOOGLE_CLIENT_SECRET environment variable."),
            ),
            "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
            Some("https://www.googleapis.com/oauth2/v3/token".to_string()),
            format!("{}/api/v1/auth/google/redirect", redirect_domain),
        ),
        get_oauth_client(
            env::var("CSH_CLIENT_ID").expect("Missing the GOOGLE_CLIENT_ID environment variable."),
            Some(
                env::var("CSH_CLIENT_SECRET")
                    .expect("Missing the GOOGLE_CLIENT_SECRET environment variable."),
            ),
            env::var("CSH_AUTH_URL").unwrap_or("".to_string()),
            Some(env::var("CSH_TOKEN_URL").unwrap_or("".to_string())),
            format!("{}/api/v1/auth/csh/redirect", redirect_domain),
        ),
    )
}
