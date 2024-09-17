use actix_session::Session;
use actix_web::{
    delete, post,
    web::{self},
    HttpResponse, Responder, Scope,
};
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};

use crate::auth::SessionAuth;
use crate::AppState;


#[derive(OpenApi)]
#[openapi(
    paths(
        create_rider,
        delete_rider
    ),
    components(schemas(Rider, CreateRider))
)]
pub struct ApiDoc;

#[derive(Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Rider {
    pub id: i32,
    pub car_id: Option<i32>,
    pub name: String,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateRider {
    pub name: String,
}

#[utoipa::path(
    params(
        ("event_id" = i32, Path, description = "ID of the Event this Rider Applies To"),
        ("car_id" = i32, Path, description = "ID of the Car this Rider Applies To")
    ),
    responses(
        (status = 200, description = "Add a rider to a car. Rider name must be current user unless car driver.", body = i32)
    )
)]
#[post("/", wrap = "SessionAuth")]
async fn create_rider(
    data: web::Data<AppState>,
    session: Session,
    rider: web::Json<CreateRider>,
    path: web::Path<(i32, i32)>,
) -> impl Responder {
    let (_event_id, car_id) = path.into_inner();
    let result = sqlx::query!(
        r#"
        INSERT INTO rider (car_id, name) VALUES ($1, $2) RETURNING id
        "#,
        car_id,
        rider.name
    )
    .fetch_one(&data.db)
    .await;

    match result {
        Ok(record) => HttpResponse::Ok().json(record.id),
        Err(_) => HttpResponse::InternalServerError().body("Failed to create car"),
    }
}

#[utoipa::path(
    params(
        ("event_id" = i32, Path, description = "ID of the Event this Rider Applies To"),
        ("car_id" = i32, Path, description = "ID of the Car this Rider Applies To")
    ),
    responses(
        (status = 200, description = "Remove rider from car. Must be done by rider or driver.", body = i32)
    )
)]
#[delete("/{rider_id}", wrap = "SessionAuth")]
async fn delete_rider(
    data: web::Data<AppState>,
    session: Session,
    path: web::Path<(i32, i32, i32)>,
) -> impl Responder {
    let (_event_id, car_id, rider_id) = path.into_inner();

    let deleted = sqlx::query!(
        "DELETE FROM rider WHERE car_id = $1 AND id = $2 RETURNING id",
        car_id,
        rider_id
    )
    .fetch_optional(&data.db)
    .await;

    match deleted {
        Ok(Some(_)) => HttpResponse::Ok().body("Rider deleted"),
        Ok(None) => HttpResponse::NotFound().body("Rider not found"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to delete rider"),
    }
}

pub fn scope() -> Scope {
    web::scope("/{car_id}/rider")
        .service(create_rider)
        .service(delete_rider)
}
