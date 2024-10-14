use actix_web::{get, web, HttpResponse, Responder, Scope};
use serde::Deserialize;
use sqlx::query_as;

use crate::app::AppState;
use crate::auth::SessionAuth;

use utoipa::OpenApi;

use crate::db::user::UserData;

#[derive(OpenApi)]
#[openapi(paths(user_search), components(schemas(UserData)))]
pub struct ApiDoc;

#[derive(Deserialize)]
struct UserSearchParams {
    query: String,
}

#[utoipa::path(
    responses(
        (status = 200, description = "Get all users matching search", body = [UserData])
    )
)]
#[get("/", wrap = "SessionAuth")]
async fn user_search(
    data: web::Data<AppState>,
    params: web::Query<UserSearchParams>,
) -> impl Responder {
    let query = format!("%{}%", params.query.to_lowercase());

    let result = query_as!(UserData, r#"SELECT id AS "id!", realm::text AS "realm!", name AS "name!", email AS "email!" FROM users WHERE LOWER(name) LIKE $1 OR LOWER(email) LIKE $1;"#, query)
        .fetch_all(&data.db)
        .await;

    match result {
        Ok(events) => HttpResponse::Ok().json(events),
        Err(_) => HttpResponse::InternalServerError().body("Failed to get events"),
    }
}

pub fn scope() -> Scope {
    web::scope("/user").service(user_search)
}
