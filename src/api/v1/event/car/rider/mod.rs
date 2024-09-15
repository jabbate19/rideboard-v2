use actix_session::Session;
use actix_web::{
    delete, get, post,
    web::{self},
    HttpResponse, Responder, Scope,
};
use serde::{Deserialize, Serialize};
use sqlx::query_as;

use crate::auth::SessionAuth;
use crate::AppState;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Rider {
    pub id: i32,
    pub car_id: Option<i32>,
    pub name: String,
}

#[derive(Deserialize)]
pub struct CreateRider {
    pub name: String,
}

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

#[get("/{rider_id}", wrap = "SessionAuth")]
async fn get_rider(
    data: web::Data<AppState>,
    session: Session,
    path: web::Path<(i32, i32, i32)>,
) -> impl Responder {
    let (event_id, car_id, rider_id) = path.into_inner();
    let result: Option<Rider> = query_as!(
        Rider,
        r#"SELECT rider.* FROM rider JOIN car ON car.id=rider.car_id WHERE car.event_id = $1 AND car.id = $2 AND rider.id = $3"#,
        event_id,
        car_id,
        rider_id
    )
    .fetch_optional(&data.db)
    .await
    .unwrap_or(None);

    match result {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().body("Rider not found"),
    }
}

#[get("/", wrap = "SessionAuth")]
async fn get_all_riders(
    data: web::Data<AppState>,
    session: Session,
    path: web::Path<(i32, i32)>,
) -> impl Responder {
    let (event_id, car_id) = path.into_inner();
    let result = query_as!(Rider, r#"SELECT rider.* FROM rider JOIN car ON car.id=rider.car_id WHERE car.event_id = $1 AND car.id = $2"#, event_id, car_id)
        .fetch_all(&data.db)
        .await;

    match result {
        Ok(events) => HttpResponse::Ok().json(events),
        Err(_) => HttpResponse::InternalServerError().body("Failed to get cars"),
    }
}

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
        .service(get_rider)
        .service(get_all_riders)
        .service(delete_rider)
}
