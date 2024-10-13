use crate::api::v1::auth::models::UserInfo;
use actix_session::Session;
use actix_web::{
    delete, get, post, put,
    web::{self},
    HttpResponse, Responder, Scope,
};
use chrono::{DateTime, Utc};
use log::error;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::query_as;

use crate::app::AppState;
use crate::auth::SessionAuth;

use utoipa::{OpenApi, ToSchema};

use super::auth::models::UserData;

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
    components(schemas(Event, CreateEvent, UserData))
)]
pub(super) struct ApiDoc;

#[derive(Serialize, Deserialize, sqlx::FromRow, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub location: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub creator: UserData,
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateEvent {
    pub name: String,
    pub location: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

fn validate_event(event: &CreateEvent) -> Vec<String> {
    let mut out = Vec::new();
    if event.name.is_empty() {
        out.push("Missing Name.".to_string());
    }
    if event.location.is_empty() {
        out.push("Missing Location.".to_string());
    }
    if event.start_time < event.end_time {
        out.push("Start date cannot be before end date.".to_string());
    }
    if event.end_time < Utc::now() {
        out.push("Event cannot be in the past.".to_string())
    }
    out
}

#[utoipa::path(
    responses(
        (status = 200, description = "Create New Event. Returns ID", body = i32)
    )
)]
#[post("/", wrap = "SessionAuth")]
async fn create_event(
    data: web::Data<AppState>,
    session: Session,
    event: web::Json<CreateEvent>,
) -> impl Responder {
    let validate = validate_event(&event);
    if !validate.is_empty() {
        return HttpResponse::BadRequest().json(json!(
            {
                "errors": validate
            }
        ));
    }
    let result = sqlx::query!(
        r#"
        INSERT INTO event (name, location, start_time, end_time, creator) VALUES ($1, $2, $3, $4, $5) RETURNING id
        "#,
        event.name, event.location, event.start_time, event.end_time, session.get::<UserInfo>("userinfo").unwrap().unwrap().id
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
        (status = 200, description = "Get event specified by ID", body = Event)
    )
)]
#[get("/{event_id}", wrap = "SessionAuth")]
async fn get_event(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let event_id = path.into_inner();
    let result: Option<Event> = query_as!(
        Event,
        r#"
            SELECT
            event.id, event.name, event.location, event.start_time, event.end_time,
            (users.id, users.realm, users.name, users.email) AS "creator!: UserData"
            FROM event
            JOIN users ON users.id = event.creator
            WHERE event.id = $1
            "#,
        event_id
    )
    .fetch_optional(&data.db)
    .await
    .unwrap_or(None);

    match result {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().body("Event not found"),
    }
}

#[derive(Deserialize)]
struct EventQueryParams {
    past: Option<bool>,
}

#[utoipa::path(
    responses(
        (status = 200, description = "Get all events", body = [Event])
    )
)]
#[get("/", wrap = "SessionAuth")]
async fn get_all_events(
    data: web::Data<AppState>,
    params: web::Query<EventQueryParams>,
) -> impl Responder {
    let past: bool = params.past.unwrap_or(false);

    let result = query_as!(
        Event,
        r#"
        SELECT
        event.id, event.name, event.location, event.start_time, event.end_time,
        (users.id, users.realm::text, users.name, users.email) AS "creator!: UserData"
        FROM event
        JOIN users ON users.id = event.creator
        WHERE (end_time >= NOW() AND $1 = False) OR (end_time < NOW() AND $1)
        ORDER BY start_time ASC
        "#,
        past
    )
    .fetch_all(&data.db)
    .await;

    match result {
        Ok(events) => HttpResponse::Ok().json(events),
        Err(e) => {
            error!("{}", e);
            HttpResponse::InternalServerError().body("Failed to get events")
        }
    }
}

#[utoipa::path(
    responses(
        (status = 200, description = "Update event information")
    )
)]
#[put("/{event_id}", wrap = "SessionAuth")]
async fn update_event(
    data: web::Data<AppState>,
    session: Session,
    path: web::Path<i32>,
    event: web::Json<CreateEvent>,
) -> impl Responder {
    let event_id = path.into_inner();

    let validate = validate_event(&event);
    if !validate.is_empty() {
        return HttpResponse::BadRequest().json(json!(
            {
                "errors": validate
            }
        ));
    }

    let updated = sqlx::query!(
        r#"
        UPDATE event SET
        name = COALESCE($1, name),
        location = COALESCE($2, location),
        start_time = COALESCE($3, start_time),
        end_time = COALESCE($4, end_time)
        WHERE id = $5 AND creator = $6
        RETURNING id
        "#,
        event.name,
        event.location,
        event.start_time,
        event.end_time,
        event_id,
        session.get::<UserInfo>("userinfo").unwrap().unwrap().id
    )
    .fetch_optional(&data.db)
    .await;

    match updated {
        Ok(Some(_)) => HttpResponse::Ok().body("Event updated successfully"),
        Ok(None) => HttpResponse::NotFound().body("Event not found or you are not the creator."),
        Err(_) => HttpResponse::InternalServerError().body("Failed to update event"),
    }
}

#[utoipa::path(
    responses(
        (status = 200, description = "Delete Event")
    )
)]
#[delete("/{event_id}", wrap = "SessionAuth")]
async fn delete_event(
    data: web::Data<AppState>,
    session: Session,
    path: web::Path<i32>,
) -> impl Responder {
    let event_id = path.into_inner();

    let deleted = sqlx::query!(
        "DELETE FROM event WHERE id = $1 AND creator = $2 RETURNING id",
        event_id,
        session.get::<UserInfo>("userinfo").unwrap().unwrap().id
    )
    .fetch_optional(&data.db)
    .await;

    match deleted {
        Ok(Some(_)) => HttpResponse::Ok().body("Event deleted"),
        Ok(None) => HttpResponse::NotFound().body("Event not found or you are not the creator."),
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
