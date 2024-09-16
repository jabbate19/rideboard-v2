use actix_session::Session;
use actix_web::{get, http::header, post, web, HttpResponse, Responder, Scope};
use models::AuthType;
use serde_json::Value;
use crate::auth::SessionAuth;

mod csh;
mod google;
mod common;
mod models;

#[post("/logout")]
async fn logout(session: Session) -> impl Responder {
    session.remove("login");
    session.remove("userinfo");
    HttpResponse::Found()
        .append_header((header::LOCATION, "/"))
        .finish()
}

#[get("/", wrap="SessionAuth")]
async fn get_user_data(session: Session) -> impl Responder {
    let out: Option<AuthType> = session.get("userinfo").unwrap();
    HttpResponse::Ok().json(out)
}

pub fn scope() -> Scope {
    web::scope("/auth")
        .service(logout)
        .service(get_user_data)
        .service(csh::scope())
        .service(google::scope())
}
