use actix_web::{web, Scope};

mod auth;
mod event;

pub fn scope() -> Scope {
    web::scope("/v1")
        .service(auth::scope())
        .service(event::scope())
}
