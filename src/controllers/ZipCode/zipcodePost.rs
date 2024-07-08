use crate::shared::auth::permissionLevelAdminMiddleware;
use crate::shared::errorHandling;
use crate::models::structsApp::AppState;
use crate::models::structsHandler::ZipCodeCreate;
use actix_web::{post, web, HttpResponse};
use actix_web_lab::middleware::from_fn;

/// Create a zip code
#[utoipa::path(
    context_path = "/zipcode",
    responses(
        (status = 201, description = "Created zip code", body = String),
        (status = 400, description = "Bad Request", body = PickUpError),
        (status = 401, description = "Unauthorized", body = PickUpError),
        (status = 500, description = "Internal Server Error", body = PickUpError)
    ),
    security(
        ("bearerAuth" = [])
    ),
    tag = "Zip Code"
)]
#[post("", wrap = "from_fn(permissionLevelAdminMiddleware)")]
pub async fn postZipCode(
    data: web::Data<AppState>,
    info: web::Json<ZipCodeCreate>,
) -> HttpResponse {
    let query: Result<_, sqlx::Error> = sqlx::query!(
        "INSERT INTO ZipCode ( Number , City) VALUES(?, ?)",
        info.Number,
        info.City
    )
    .execute(&data.pool)
    .await;

    match query {
        Err(e) => {
            return errorHandling::getHRFromErrorDatabase(e);
        }
        Ok(_) => {
            return HttpResponse::Created()
                .content_type("application/json")
                .finish();
        }
    }
}
