use actix_web::{web, Scope};

mod login;
mod event;
mod car;
mod rider;

pub fn scope() -> Scope {
    web::scope("/v1")
        .service(login::scope())
        .service(event::scope())
        .service(car::scope())
        .service(rider::scope())
}
