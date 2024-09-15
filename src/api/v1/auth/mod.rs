use actix_session::Session;
use actix_web::{http::header, post, web, HttpResponse, Responder, Scope};

mod csh;
mod google;

#[post("/logout")]
async fn logout(session: Session) -> impl Responder {
    session.remove("login");
    HttpResponse::Found()
        .append_header((header::LOCATION, "/"))
        .finish()
}

pub fn scope() -> Scope {
    web::scope("/auth")
        .service(logout)
        .service(csh::scope())
        .service(google::scope())
}
