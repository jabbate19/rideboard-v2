use crate::{
    api::v1::auth::models::UserInfo,
    db::event::{Event, EventData},
};
use actix_session::Session;
use actix_web::{
    delete, get, post, put,
    web::{self},
    HttpResponse, Responder, Scope,
};
use log::error;
use serde::Deserialize;
use serde_json::json;

use crate::app::AppState;
use crate::auth::SessionAuth;

use utoipa::OpenApi;

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
    components(schemas(Event, EventData, UserData))
)]
pub(super) struct ApiDoc;

#[utoipa::path(
    responses(
        (status = 200, description = "Create New Event. Returns ID", body = i32)
    )
)]
#[post("/", wrap = "SessionAuth")]
async fn create_event(
    data: web::Data<AppState>,
    session: Session,
    event: web::Json<EventData>,
) -> impl Responder {
    if let Err(errs) = event.validate() {
        return HttpResponse::BadRequest().json(json!(
            {
                "errors": errs
            }
        ));
    }
    let result = Event::insert_new(
        &event,
        session.get::<UserInfo>("userinfo").unwrap().unwrap().id,
        &data.db,
    )
    .await;

    match result {
        Ok(record) => HttpResponse::Ok().json(record.id),
        Err(err) => {
            error!("{}", err);
            HttpResponse::InternalServerError().body("Failed to create event")
        },
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
    let result = Event::select_one(event_id, &data.db).await.unwrap();

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

    let result = crate::db::event::Event::select_all(past, &data.db).await;

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
    event: web::Json<EventData>,
) -> impl Responder {
    let event_id = path.into_inner();

    if let Err(errs) = event.validate() {
        return HttpResponse::BadRequest().json(json!(
            {
                "errors": errs
            }
        ));
    }

    let updated = Event::update(
        event_id,
        session.get::<UserInfo>("userinfo").unwrap().unwrap().id,
        &event,
        &data.db,
    )
    .await;

    match updated {
        Ok(Some(_)) => HttpResponse::Ok().body("Event updated successfully"),
        Ok(None) => HttpResponse::NotFound().body("Event not found or you are not the creator."),
        Err(err) => {
            error!("{}", err);
            HttpResponse::InternalServerError().body("Failed to update event")
        },
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

    let deleted = Event::delete(
        event_id,
        session.get::<UserInfo>("userinfo").unwrap().unwrap().id,
        &data.db,
    )
    .await;

    match deleted {
        Ok(Some(_)) => HttpResponse::Ok().body("Event deleted"),
        Ok(None) => HttpResponse::NotFound().body("Event not found or you are not the creator."),
        Err(err) => {
            error!("{}", err);
            HttpResponse::InternalServerError().body("Failed to delete event")
        },
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
