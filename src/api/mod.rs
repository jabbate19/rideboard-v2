use actix_web::{body::BoxBody, web, HttpResponse, Scope};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod v1;

#[derive(OpenApi)]
#[openapi(
    nest(
        (path = "/api/v1", api = v1::ApiDoc)
    ),
)]
pub(super) struct ApiDoc;

pub async fn open_api_spec() -> HttpResponse<BoxBody> {
    let openapi = ApiDoc::openapi();
    HttpResponse::Ok().json(openapi)
}

pub fn scope() -> Scope {
    web::scope("/api").service(v1::scope()).service(
        web::scope("/docs")
            .route("/openapi.json", web::get().to(open_api_spec))
            .service(SwaggerUi::new("/{_:.*}").url("/api/docs/openapi.json", Default::default())),
    )
}
