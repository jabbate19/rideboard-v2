use actix_session::Session;
use actix_web::{
    delete, get, post, put,
    web::{self},
    HttpResponse, Responder, Scope,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::query_as;
use crate::auth::SessionAuth;
use crate::AppState;
use crate::api::v1::auth::models::UserInfo;
use utoipa::{OpenApi, ToSchema};

use log::error;

mod rider;

#[derive(OpenApi)]
#[openapi(
    nest(
        (path = "/{car_id}/rider", api = rider::ApiDoc),
    ),
    paths(
        create_car,
        get_car,
        get_all_cars,
        update_car,
        delete_car
    ),
    components(schemas(CarData, CreateCar, UpdateCar))
)]
pub struct ApiDoc;

#[derive(Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Car {
    pub id: i32,
    pub event_id: Option<i32>,
    pub driver: String,
    pub max_capacity: i32,
    pub departure_time: DateTime<Utc>,
    pub return_time: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct CarData {
    pub id: i32,
    pub event_id: Option<i32>,
    pub driver: String,
    pub riders: Vec<String>,
    pub max_capacity: i32,
    pub departure_time: DateTime<Utc>,
    pub return_time: DateTime<Utc>,
    pub comment: String,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateCar {
    pub max_capacity: i32,
    pub departure_time: DateTime<Utc>,
    pub return_time: DateTime<Utc>,
}

#[derive(Deserialize, ToSchema)]
struct UpdateCar {
    pub driver: Option<String>,
    pub max_capacity: Option<i32>,
    pub departure_time: Option<DateTime<Utc>>,
    pub return_time: Option<DateTime<Utc>>,
}

#[utoipa::path(
    params(
        ("event_id" = i32, Path, description = "ID of the Event this Car Applies To")
    ),
    responses(
        (status = 200, description = "Create new Car for Event.", body = i32)
    )
)]
#[post("/", wrap = "SessionAuth")]
async fn create_car(
    data: web::Data<AppState>,
    session: Session,
    car: web::Json<CreateCar>,
    path: web::Path<i32>,
) -> impl Responder {
    let event_id: i32 = path.into_inner();
    let result = sqlx::query!(
        r#"
        INSERT INTO car (event_id, driver, max_capacity, departure_time, return_time) VALUES ($1, $2, $3, $4, $5) RETURNING id
        "#,
        event_id, session.get::<UserInfo>("userdata").unwrap().unwrap().id, car.max_capacity, car.departure_time, car.return_time
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
        ("event_id" = i32, Path, description = "ID of the Event this Car Applies To")
    ),
    responses(
        (status = 200, description = "Get car by ID", body = CarData)
    )
)]
#[get("/{car_id}", wrap = "SessionAuth")]
async fn get_car(
    data: web::Data<AppState>,
    session: Session,
    path: web::Path<(i32, i32)>,
) -> impl Responder {
    let (event_id, car_id) = path.into_inner();
    let result: Option<CarData> = query_as!(
        CarData,
        r#"SELECT car.*, ARRAY_AGG(rider.name) as "riders!: Vec<String>" FROM car LEFT JOIN rider on car.id = rider.car_id WHERE event_id = $1 AND car.id = $2 GROUP BY car.id"#,
        event_id,
        car_id
    )
    .fetch_optional(&data.db)
    .await
    .unwrap_or(None);

    match result {
        Some(car) => HttpResponse::Ok().json(car),
        None => HttpResponse::NotFound().body("Car not found"),
    }
}

#[utoipa::path(
    params(
        ("event_id" = i32, Path, description = "ID of the Event this Car Applies To")
    ),
    responses(
        (status = 200, description = "Get all cars for event", body = [CarData])
    )
)]
#[get("/", wrap = "SessionAuth")]
async fn get_all_cars(
    data: web::Data<AppState>,
    session: Session,
    path: web::Path<i32>,
) -> impl Responder {
    let event_id: i32 = path.into_inner();
    let result = query_as!(CarData, r#"SELECT car.*, ARRAY_AGG(rider.name) as "riders!: Vec<String>" FROM car LEFT JOIN rider on car.id = rider.car_id WHERE event_id = $1 GROUP BY car.id"#, event_id)
        .fetch_all(&data.db)
        .await;

    match result {
        Ok(cars) => HttpResponse::Ok().json(cars),
        Err(e) => {
            error!("{}", e);
            HttpResponse::InternalServerError().body("Failed to get cars")
        },
    }
}

#[utoipa::path(
    params(
        ("event_id" = i32, Path, description = "ID of the Event this Car Applies To")
    ),
    responses(
        (status = 200, description = "Update Car")
    )
)]
#[put("/{car_id}", wrap = "SessionAuth")]
async fn update_car(
    data: web::Data<AppState>,
    session: Session,
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

#[utoipa::path(
    params(
        ("event_id" = i32, Path, description = "ID of the Event this Car Applies To")
    ),
    responses(
        (status = 200, description = "Delete Car")
    )
)]
#[delete("/{car_id}", wrap = "SessionAuth")]
async fn delete_car(
    data: web::Data<AppState>,
    session: Session,
    path: web::Path<(i32, i32)>,
) -> impl Responder {
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
