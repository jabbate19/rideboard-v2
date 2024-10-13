use actix_web::{web, Scope};
use utoipa::OpenApi;

mod auth;
mod event;
mod user;

#[derive(OpenApi)]
#[openapi(
    nest(
        (path = "/auth", api = auth::ApiDoc),
        (path = "/event", api = event::ApiDoc),
        (path = "/user", api = user::ApiDoc)
    ),
)]
pub(super) struct ApiDoc;

pub fn scope() -> Scope {
    web::scope("/v1")
        .service(auth::scope())
        .service(event::scope())
        .service(user::scope())
}
