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

mod car;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub location: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct CreateEvent {
    pub name: String,
    pub location: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

#[derive(Deserialize)]
struct UpdateEvent {
    pub name: Option<String>,
    pub location: Option<String>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
}

#[post("/", wrap = "SessionAuth")]
async fn create_event(
    data: web::Data<AppState>,
    session: Session,
    event: web::Json<CreateEvent>,
) -> impl Responder {
    let result = sqlx::query!(
        r#"
        INSERT INTO event (name, location, start_time, end_time) VALUES ($1, $2, $3, $4) RETURNING id
        "#,
        event.name, event.location, event.start_time, event.end_time
    )
    .fetch_one(&data.db)
    .await;

    match result {
        Ok(record) => HttpResponse::Ok().json(record.id),
        Err(_) => HttpResponse::InternalServerError().body("Failed to create event"),
    }
}

#[get("/{event_id}", wrap = "SessionAuth")]
async fn get_event(
    data: web::Data<AppState>,
    session: Session,
    path: web::Path<i32>,
) -> impl Responder {
    let event_id = path.into_inner();
    let result: Option<Event> = query_as!(Event, r#"SELECT * FROM event WHERE id = $1"#, event_id)
        .fetch_optional(&data.db)
        .await
        .unwrap_or(None);

    match result {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().body("Event not found"),
    }
}

#[get("/", wrap = "SessionAuth")]
async fn get_all_events(data: web::Data<AppState>, session: Session) -> impl Responder {
    let result = query_as!(Event, r#"SELECT * FROM event"#)
        .fetch_all(&data.db)
        .await;

    match result {
        Ok(events) => HttpResponse::Ok().json(events),
        Err(_) => HttpResponse::InternalServerError().body("Failed to get events"),
    }
}

#[put("/{event_id}", wrap = "SessionAuth")]
async fn update_event(
    data: web::Data<AppState>,
    session: Session,
    path: web::Path<i32>,
    event: web::Json<UpdateEvent>,
) -> impl Responder {
    let event_id = path.into_inner();

    let updated = sqlx::query!(
        r#"
        UPDATE event SET
        name = COALESCE($1, name),
        location = COALESCE($2, location),
        start_time = COALESCE($3, start_time),
        end_time = COALESCE($4, end_time)
        WHERE id = $5 RETURNING id
        "#,
        event.name,
        event.location,
        event.start_time,
        event.end_time,
        event_id
    )
    .fetch_optional(&data.db)
    .await;

    match updated {
        Ok(Some(_)) => HttpResponse::Ok().body("Event updated successfully"),
        Ok(None) => HttpResponse::NotFound().body("Event not found"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to update event"),
    }
}

#[delete("/{event_id}", wrap = "SessionAuth")]
async fn delete_event(
    data: web::Data<AppState>,
    session: Session,
    path: web::Path<i32>,
) -> impl Responder {
    let event_id = path.into_inner();

    let deleted = sqlx::query!("DELETE FROM event WHERE id = $1 RETURNING id", event_id)
        .fetch_optional(&data.db)
        .await;

    match deleted {
        Ok(Some(_)) => HttpResponse::Ok().body("Event deleted"),
        Ok(None) => HttpResponse::NotFound().body("Event not found"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to delete event"),
    }
}

pub fn scope() -> Scope {
    web::scope("/event")
        .service(create_event)
        .service(get_event)
        .service(get_all_events)
        .service(update_event)
        .service(delete_event)
        .service(car::scope())
}
