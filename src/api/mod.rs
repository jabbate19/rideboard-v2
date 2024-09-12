use actix_web::{web, Scope};

mod v1;

pub fn scope() -> Scope {
    web::scope("/api").service(v1::scope())
}
