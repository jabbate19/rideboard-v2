use actix_session::Session;
use actix_web::{get, http::header, post, web, HttpResponse, Responder, Scope};
use models::AuthType;
use crate::auth::SessionAuth;
use crate::api::v1::auth::models::UserInfo;
use utoipa::{OpenApi};

mod csh;
mod google;
mod common;
mod models;

#[utoipa::path(
    responses(
        (status = 200, description = "List current todo items")
    )
)]
#[post("/logout")]
async fn logout(session: Session) -> impl Responder {
    session.remove("login");
    session.remove("userinfo");
    HttpResponse::Found()
        .append_header((header::LOCATION, "/"))
        .finish()
}

#[utoipa::path(
    responses(
        (status = 200, description = "List current todo items")
    )
)]
#[get("/", wrap="SessionAuth")]
async fn get_user_data(session: Session) -> impl Responder {
    let out: Option<AuthType> = session.get("userinfo").unwrap();
    HttpResponse::Ok().json(out.map(UserInfo::from))
}

#[derive(OpenApi)]
#[openapi(
    paths(
        logout,
        get_user_data,
    ),
    nest(
        (path = "/csh", api = csh::ApiDoc),
        (path = "/google", api = google::ApiDoc)
    ),
)]
pub(super) struct ApiDoc;

pub fn scope() -> Scope {
    web::scope("/auth")
        .service(logout)
        .service(get_user_data)
        .service(csh::scope())
        .service(google::scope())
}
