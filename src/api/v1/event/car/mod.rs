use crate::api::v1::auth::models::UserInfo;
use crate::app::{AppState, MultipleRiderChange, RedisJob};
use crate::db::car::{Car, CarData};
use crate::{api::v1::auth::models::UserData, auth::SessionAuth};
use actix_session::Session;
use actix_web::{
    delete, get, post, put,
    web::{self},
    HttpResponse, Responder, Scope,
};
use redis_work_queue::{Item, WorkQueue};
use serde_json::json;
use sqlx::query;
use utoipa::OpenApi;

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
    components(schemas(Car, CarData, UserData))
)]
pub struct ApiDoc;

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
    car: web::Json<CarData>,
    path: web::Path<i32>,
) -> impl Responder {
    let event_id: i32 = path.into_inner();
    let user_id = session.get::<UserInfo>("userinfo").unwrap().unwrap().id;
    let mut tx = data.db.begin().await.unwrap();
    let other_cars = Car::select_all(event_id, &mut *tx).await.unwrap();
    if let Err(errs) = car.validate(&user_id, other_cars) {
        tx.rollback().await.unwrap();
        return HttpResponse::BadRequest().json(json!({
            "errors": errs
        }));
    }

    let record = Car::insert_new(event_id, user_id, &car, &mut *tx)
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
    let work_queue = WorkQueue::new(data.work_queue_key.clone());
    let item = Item::from_json_data(&RedisJob::RiderUpdate(MultipleRiderChange {
        event_id,
        car_id: record.id,
        old_riders: Vec::new(),
        new_riders: car.riders.clone(),
    }))
    .unwrap();
    let mut redis = data.redis.lock().unwrap().clone();
    work_queue
        .add_item(&mut redis, &item)
        .await
        .expect("failed to add item to work queue");
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
    let result = Car::select_one(event_id, car_id, &data.db).await;

    match result {
        Ok(Some(car)) => HttpResponse::Ok().json(car),
        Ok(None) => HttpResponse::NotFound().body("Car not found"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to get Car"),
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
    let result = Car::select_all(event_id, &data.db).await;

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
    car: web::Json<CarData>,
) -> impl Responder {
    let (event_id, car_id) = path.into_inner();
    let user_id = session.get::<UserInfo>("userinfo").unwrap().unwrap().id;
    let mut tx = data.db.begin().await.unwrap();
    let other_cars = Car::select_all(event_id, &mut *tx)
        .await
        .unwrap()
        .into_iter()
        .filter(|car| car.id != car_id)
        .collect();
    if let Err(errs) = car.validate(&user_id, other_cars) {
        tx.rollback().await.unwrap();
        return HttpResponse::BadRequest().json(json!({
            "errors": errs
        }));
    }
    let updated = Car::update(car_id, event_id, user_id, &car, &mut *tx).await;

    match updated {
        Ok(Some(_)) => {}
        Ok(None) => {
            tx.rollback().await.unwrap();
            return HttpResponse::NotFound().body("Car not found or you are not the driver.");
        }
        Err(err) => {
            tx.rollback().await.unwrap();
            error!("{}", err);
            return HttpResponse::InternalServerError().body("Failed to update car");
        }
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

    let work_queue = WorkQueue::new(data.work_queue_key.clone());
    let item = Item::from_json_data(&RedisJob::RiderUpdate(MultipleRiderChange {
        event_id,
        car_id,
        old_riders: current_riders,
        new_riders: car.riders.clone(),
    }))
    .unwrap();
    let mut redis = data.redis.lock().unwrap().clone();
    work_queue
        .add_item(&mut redis, &item)
        .await
        .expect("failed to add item to work queue");
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
    let user_id = session.get::<UserInfo>("userinfo").unwrap().unwrap().id;

    let deleted = Car::delete(car_id, event_id, user_id, &data.db).await;

    match deleted {
        Ok(Some(_)) => HttpResponse::Ok().body("Car deleted"),
        Ok(None) => HttpResponse::NotFound().body("Car not found or you are not the driver."),
        Err(err) => {
            error!("{}", err);
            HttpResponse::InternalServerError().body("Failed to delete car")
        }
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
