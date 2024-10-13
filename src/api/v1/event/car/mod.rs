use crate::api::v1::auth::models::UserInfo;
use crate::AppState;
use crate::{api::v1::auth::models::UserData, auth::SessionAuth};
use actix_session::Session;
use actix_web::{
    delete, get, post, put,
    web::{self},
    HttpResponse, Responder, Scope,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{query, query_as};
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
    components(schemas(CarData, CreateCar, UserData))
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
#[serde(rename_all = "camelCase")]
pub struct CarData {
    pub id: i32,
    pub event_id: Option<i32>,
    pub driver: UserData,
    pub riders: Option<Vec<UserData>>,
    pub max_capacity: i32,
    pub departure_time: DateTime<Utc>,
    pub return_time: DateTime<Utc>,
    pub comment: String,
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateCar {
    pub max_capacity: i32,
    pub departure_time: DateTime<Utc>,
    pub return_time: DateTime<Utc>,
    pub comment: String,
    pub riders: Vec<String>,
}

fn validate_car(car: &CreateCar, user: &String, other_cars: Vec<CarData>) -> Vec<String> {
    let mut out = Vec::new();
    if car.return_time < car.departure_time {
        out.push("Return time cannot be before departure.".to_string())
    }
    if car.departure_time < Utc::now() {
        out.push("Car cannot leave in the past.".to_string());
    }
    if car.max_capacity < 0 {
        out.push("Capacity must be greater than or equal to 0".to_string());
    }
    if car.riders.len() > (car.max_capacity as usize) {
        out.push("You have too many riders for your capacity.".to_string());
    }
    if car.riders.contains(user) {
        out.push("You cannot be a rider in your own car.".to_string());
    }
    let other_car_members: Vec<String> = other_cars
        .iter()
        .flat_map(|car| {
            let mut out = car.riders.as_ref().unwrap().clone();
            out.push(car.driver.clone());
            out
        })
        .map(|user| user.id)
        .collect();
    for rider in car.riders.iter() {
        if other_car_members.contains(&rider) {
            out.push(format!(
                "{} is already in another car or is a driver.",
                rider
            ))
        }
    }
    out
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
    let user_id = session.get::<UserInfo>("userinfo").unwrap().unwrap().id;
    let mut tx = data.db.begin().await.unwrap();
    let other_cars = query_as!(
        CarData,
        r#"SELECT car.id, car.event_id, car.max_capacity, car.departure_time, car.return_time, car.comment,
        (driverUser.id, driverUser.realm::text, driverUser.name, driverUser.email) AS "driver!: UserData",
        ARRAY_REMOVE(ARRAY_AGG(
            CASE WHEN riderUser.id IS NOT NULL
            THEN (riderUser.id, riderUser.realm::text, riderUser.name, riderUser.email)
            END
        ), NULL) as "riders!: Vec<UserData>"
        FROM car
        JOIN users driverUser ON car.driver = driverUser.id
        LEFT JOIN rider on car.id = rider.car_id
        LEFT JOIN users riderUser ON rider.rider = riderUser.id
        WHERE event_id = $1 GROUP BY car.id, driverUser.id"#,
        event_id)
        .fetch_all(&mut *tx)
        .await.unwrap();
    let validate = validate_car(&car, &user_id, other_cars);
    if !validate.is_empty() {
        return HttpResponse::BadRequest().json(json!({
            "errors": validate
        }));
    }

    let record = query!(
        r#"
        INSERT INTO car (event_id, driver, max_capacity, departure_time, return_time, comment)
        VALUES ($1, $2, $3, $4, $5, $6) RETURNING id
        "#,
        event_id,
        user_id,
        car.max_capacity,
        car.departure_time,
        car.return_time,
        car.comment
    )
    .fetch_one(&mut *tx)
    .await
    .unwrap();

    let _ = query!(
        r#"
        INSERT INTO rider (car_id, rider) SELECT $1, * FROM UNNEST($2::VARCHAR[])
        "#,
        record.id,
        &car.riders
    )
    .execute(&mut *tx)
    .await
    .unwrap();
    tx.commit().await.unwrap();
    HttpResponse::Ok().json(record.id)
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
async fn get_car(data: web::Data<AppState>, path: web::Path<(i32, i32)>) -> impl Responder {
    let (event_id, car_id) = path.into_inner();
    let result: Option<CarData> = query_as!(
        CarData,
        r#"SELECT car.id, car.event_id, car.max_capacity, car.departure_time, car.return_time, car.comment,
        (driverUser.id, driverUser.realm::text, driverUser.name, driverUser.email) AS "driver!: UserData",
        ARRAY_REMOVE(ARRAY_AGG(
            CASE WHEN riderUser.id IS NOT NULL
            THEN (riderUser.id, riderUser.realm::text, riderUser.name, riderUser.email)
            END
        ), NULL) as "riders!: Vec<UserData>"
        FROM car
        JOIN users driverUser ON car.driver = driverUser.id
        LEFT JOIN rider on car.id = rider.car_id
        LEFT JOIN users riderUser ON rider.rider = riderUser.id
        WHERE event_id = $1 AND car.id = $2 GROUP BY car.id, driverUser.id"#,
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
async fn get_all_cars(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let event_id: i32 = path.into_inner();
    let result = query_as!(
        CarData,
        r#"SELECT car.id, car.event_id, car.max_capacity, car.departure_time, car.return_time, car.comment,
        (driverUser.id, driverUser.realm::text, driverUser.name, driverUser.email) AS "driver!: UserData",
        ARRAY_REMOVE(ARRAY_AGG(
            CASE WHEN riderUser.id IS NOT NULL
            THEN (riderUser.id, riderUser.realm::text, riderUser.name, riderUser.email)
            END
        ), NULL) as "riders!: Vec<UserData>"
        FROM car
        JOIN users driverUser ON car.driver = driverUser.id
        LEFT JOIN rider on car.id = rider.car_id
        LEFT JOIN users riderUser ON rider.rider = riderUser.id
        WHERE event_id = $1 GROUP BY car.id, driverUser.id"#,
        event_id)
        .fetch_all(&data.db)
        .await;

    match result {
        Ok(cars) => HttpResponse::Ok().json(cars),
        Err(e) => {
            error!("{}", e);
            HttpResponse::InternalServerError().body("Failed to get cars")
        }
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
    car: web::Json<CreateCar>,
) -> impl Responder {
    let (event_id, car_id) = path.into_inner();
    let user_id = session.get::<UserInfo>("userinfo").unwrap().unwrap().id;
    let mut tx = data.db.begin().await.unwrap();
    let other_cars = query_as!(
        CarData,
        r#"SELECT car.id, car.event_id, car.max_capacity, car.departure_time, car.return_time, car.comment,
        (driverUser.id, driverUser.realm::text, driverUser.name, driverUser.email) AS "driver!: UserData",
        ARRAY_REMOVE(ARRAY_AGG(
            CASE WHEN riderUser.id IS NOT NULL
            THEN (riderUser.id, riderUser.realm::text, riderUser.name, riderUser.email)
            END
        ), NULL) as "riders!: Vec<UserData>"
        FROM car
        JOIN users driverUser ON car.driver = driverUser.id
        LEFT JOIN rider on car.id = rider.car_id
        LEFT JOIN users riderUser ON rider.rider = riderUser.id
        WHERE event_id = $1 AND car_id != $2 GROUP BY car.id, driverUser.id"#,
        event_id, car_id)
        .fetch_all(&mut *tx)
        .await.unwrap();
    let validate = validate_car(&car, &user_id, other_cars);
    if !validate.is_empty() {
        return HttpResponse::BadRequest().json(json!({
            "errors": validate
        }));
    }
    let updated = query!(
        r#"
        UPDATE car SET
        max_capacity = COALESCE($1, max_capacity),
        departure_time = COALESCE($2, departure_time),
        return_time = COALESCE($3, return_time),
        comment = COALESCE($4, comment)
        WHERE event_id = $5 AND id = $6 AND driver = $7 RETURNING id
        "#,
        car.max_capacity,
        car.departure_time,
        car.return_time,
        car.comment,
        event_id,
        car_id,
        user_id
    )
    .fetch_optional(&mut *tx)
    .await;

    match updated {
        Ok(Some(_)) => {}
        Ok(None) => {
            return HttpResponse::NotFound().body("Car not found or you are not the driver.")
        }
        Err(_) => return HttpResponse::InternalServerError().body("Failed to update car"),
    }

    // Used for sending pings
    let current_riders: Vec<String> = query!(
        r#"DELETE FROM rider WHERE car_id = $1 RETURNING rider"#,
        car_id
    )
    .fetch_all(&mut *tx)
    .await
    .unwrap()
    .iter()
    .map(|record| record.rider.clone())
    .collect();

    let _ = query!(
        r#"
        INSERT INTO rider (car_id, rider) SELECT $1, * FROM UNNEST($2::VARCHAR[])
        "#,
        car_id,
        &car.riders
    )
    .execute(&mut *tx)
    .await
    .unwrap();
    tx.commit().await.unwrap();

    HttpResponse::Ok().body("Car updated successfully")
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

    let deleted = query!(
        "DELETE FROM car WHERE event_id = $1 AND id = $2 AND driver = $3 RETURNING id",
        event_id,
        car_id,
        session.get::<UserInfo>("userinfo").unwrap().unwrap().id
    )
    .fetch_optional(&data.db)
    .await;

    match deleted {
        Ok(Some(_)) => HttpResponse::Ok().body("Car deleted"),
        Ok(None) => HttpResponse::NotFound().body("Car not found or you are not the driver."),
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
