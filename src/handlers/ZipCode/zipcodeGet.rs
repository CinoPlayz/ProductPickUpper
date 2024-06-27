use crate::shared::auth::{
    permissionLevelAdminMiddleware, permissionLevelUserMiddleware,
};
use crate::shared::errorHandling;
use crate::shared::structs::structsApp::AppState;
use crate::shared::structs::structsHandler::ZipCode;
use actix_web::{get, web, HttpResponse};
use actix_web_lab::middleware::from_fn;

/// Get all zip codes
#[utoipa::path(
    context_path = "/",
    responses(
        (status = 200, description = "Returns all zip codes", body = Vec<ZipCode>),
        (status = 401, description = "Unauthorized", body = PickUpError),
        (status = 500, description = "Internal Server Error", body = PickUpError)
    ),
    security(
        ("bearerAuth" = [])
    ),
    tag = "Zip Code"
)]
#[get("", wrap = "from_fn(permissionLevelUserMiddleware)")]
pub async fn getAllZipCodes(data: web::Data<AppState>) -> HttpResponse {
    let query: Result<Vec<ZipCode>, sqlx::Error> = sqlx::query_as!(
        ZipCode,
        "SELECT Id AS 'ZipCodeId', Number, City FROM ZipCode"
    )
    .fetch_all(&data.pool)
    .await;

    match query {
        //e.g. If connection to database is lost
        Err(e) => {
            return errorHandling::getHRFromErrorInternal(e);
        }
        Ok(users) => HttpResponse::Ok()
            .content_type("application/json")
            .json(&users),
    }
}

/// Get zip code by Id
#[utoipa::path(
    context_path = "/",
    responses(
        (status = 200, description = "Returns all users", body = ZipCode),
        (status = 401, description = "Unauthorized", body = PickUpError),
        (status = 500, description = "Internal Server Error", body = PickUpError)
    ),
    security(
        ("bearerAuth" = [])
    ),
    tag = "Zip Code"
)]
#[get("/{id}", wrap = "from_fn(permissionLevelAdminMiddleware)")]
pub async fn getZipCodeById(data: web::Data<AppState>, path: web::Path<String>) -> HttpResponse {
    let uuid = path.into_inner();

    let query: Result<Vec<ZipCode>, sqlx::Error> = sqlx::query_as!(
        ZipCode,
        "SELECT Id AS 'ZipCodeId', Number, City FROM ZipCode WHERE Id=?",
        uuid
    )
    .fetch_all(&data.pool)
    .await;

    match query {
        //e.g. If connection to database is lost
        Err(e) => {
            return errorHandling::getHRFromErrorInternal(e);
        }
        Ok(users) => HttpResponse::Ok()
            .content_type("application/json")
            .json(&users),
    }
}
