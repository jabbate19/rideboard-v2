use actix_web::{
    delete, get, post, put,
    web::{self},
    HttpResponse, Responder, Scope,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::query_as;

use crate::AppState;

mod rider;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Car {
    pub id: i32,
    pub event_id: Option<i32>,
    pub driver: String,
    pub max_capacity: i32,
    pub departure_time: DateTime<Utc>,
    pub return_time: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct CreateCar {
    pub driver: String,
    pub max_capacity: i32,
    pub departure_time: DateTime<Utc>,
    pub return_time: DateTime<Utc>,
}

#[derive(Deserialize)]
struct UpdateCar {
    pub driver: Option<String>,
    pub max_capacity: Option<i32>,
    pub departure_time: Option<DateTime<Utc>>,
    pub return_time: Option<DateTime<Utc>>,
}

#[post("/")]
async fn create_car(
    data: web::Data<AppState>,
    car: web::Json<CreateCar>,
    path: web::Path<i32>,
) -> impl Responder {
    let event_id: i32 = path.into_inner();
    let result = sqlx::query!(
        r#"
        INSERT INTO car (event_id, driver, max_capacity, departure_time, return_time) VALUES ($1, $2, $3, $4, $5) RETURNING id
        "#,
        event_id, car.driver, car.max_capacity, car.departure_time, car.return_time
    )
    .fetch_one(&data.db)
    .await;

    match result {
        Ok(record) => HttpResponse::Ok().json(record.id),
        Err(_) => HttpResponse::InternalServerError().body("Failed to create car"),
    }
}

#[get("/{car_id}")]
async fn get_car(data: web::Data<AppState>, path: web::Path<(i32, i32)>) -> impl Responder {
    let (event_id, car_id) = path.into_inner();
    let result: Option<Car> = query_as!(
        Car,
        r#"SELECT * FROM car WHERE event_id = $1 AND id = $2"#,
        event_id,
        car_id
    )
    .fetch_optional(&data.db)
    .await
    .unwrap_or(None);

    match result {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().body("Car not found"),
    }
}

#[get("/")]
async fn get_all_cars(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let event_id: i32 = path.into_inner();
    let result = query_as!(Car, r#"SELECT * FROM car WHERE event_id = $1"#, event_id)
        .fetch_all(&data.db)
        .await;

    match result {
        Ok(events) => HttpResponse::Ok().json(events),
        Err(_) => HttpResponse::InternalServerError().body("Failed to get cars"),
    }
}

#[put("/{car_id}")]
async fn update_car(
    data: web::Data<AppState>,
    path: web::Path<(i32, i32)>,
    car: web::Json<UpdateCar>,
) -> impl Responder {
    let (event_id, car_id) = path.into_inner();

    let updated = sqlx::query!(
        r#"
        UPDATE car SET
        driver = COALESCE($1, driver),
        max_capacity = COALESCE($2, max_capacity),
        departure_time = COALESCE($3, departure_time),
        return_time = COALESCE($4, return_time)
        WHERE event_id = $5 AND id = $6 RETURNING id
        "#,
        car.driver,
        car.max_capacity,
        car.departure_time,
        car.return_time,
        event_id,
        car_id
    )
    .fetch_optional(&data.db)
    .await;

    match updated {
        Ok(Some(_)) => HttpResponse::Ok().body("Car updated successfully"),
        Ok(None) => HttpResponse::NotFound().body("Car not found"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to update car"),
    }
}

#[delete("/{car_id}")]
async fn delete_car(data: web::Data<AppState>, path: web::Path<(i32, i32)>) -> impl Responder {
    let (event_id, car_id) = path.into_inner();

    let deleted = sqlx::query!(
        "DELETE FROM car WHERE event_id = $1 AND id = $2 RETURNING id",
        event_id,
        car_id
    )
    .fetch_optional(&data.db)
    .await;

    match deleted {
        Ok(Some(_)) => HttpResponse::Ok().body("Car deleted"),
        Ok(None) => HttpResponse::NotFound().body("Car not found"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to delete car"),
    }
}

pub fn scope() -> Scope {
    web::scope("/{event_id}/car")
        .service(create_car)
        .service(get_car)
        .service(get_all_cars)
        .service(update_car)
        .service(delete_car)
        .service(rider::scope())
}
