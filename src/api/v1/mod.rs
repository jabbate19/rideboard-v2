use actix_web::{web, Scope};

mod event;
mod login;

pub fn scope() -> Scope {
    web::scope("/v1")
        .service(login::scope())
        .service(event::scope())
}
