use actix_web::{web, Scope};
use utoipa::OpenApi;

mod auth;
mod event;

#[derive(OpenApi)]
#[openapi(
    nest(
        (path = "/auth", api = auth::ApiDoc),
        (path = "/event", api = event::ApiDoc)
    ),
)]
pub(super) struct ApiDoc;

pub fn scope() -> Scope {
    web::scope("/v1")
        .service(auth::scope())
        .service(event::scope())
}
