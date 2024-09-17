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

use utoipa::{OpenApi, ToSchema};

mod car;

#[derive(OpenApi)]
#[openapi(
    nest(
        (path = "/{event_id}/car", api = car::ApiDoc),
    ),
    paths(
        create_event,
        get_event,
        get_all_events,
        update_event,
        delete_event
    ),
    components(schemas(Event, CreateEvent, UpdateEvent))
)]
pub(super) struct ApiDoc;

#[derive(Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub location: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub creator: String,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateEvent {
    pub name: String,
    pub location: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub creator: String,
}

#[derive(Deserialize, ToSchema)]
struct UpdateEvent {
    pub name: Option<String>,
    pub location: Option<String>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
}

#[utoipa::path(
    responses(
        (status = 200, description = "List current todo items", body = i32)
    )
)]
#[post("/", wrap = "SessionAuth")]
async fn create_event(
    data: web::Data<AppState>,
    session: Session,
    event: web::Json<CreateEvent>,
) -> impl Responder {
    let result = sqlx::query!(
        r#"
        INSERT INTO event (name, location, start_time, end_time, creator) VALUES ($1, $2, $3, $4, $5) RETURNING id
        "#,
        event.name, event.location, event.start_time, event.end_time, event.creator
    )
    .fetch_one(&data.db)
    .await;

    match result {
        Ok(record) => HttpResponse::Ok().json(record.id),
        Err(_) => HttpResponse::InternalServerError().body("Failed to create event"),
    }
}

#[utoipa::path(
    responses(
        (status = 200, description = "List current todo items", body = Event)
    )
)]
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

#[utoipa::path(
    responses(
        (status = 200, description = "List current todo items", body = [Event])
    )
)]
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

#[utoipa::path(
    responses(
        (status = 200, description = "List current todo items")
    )
)]
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

#[utoipa::path(
    responses(
        (status = 200, description = "List current todo items")
    )
)]
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
