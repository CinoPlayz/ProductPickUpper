use crate::shared::auth::permissionLevelAdminMiddleware;
use crate::shared::errorHandling;
use crate::models::structsApp::{AppState, PickUpError, PickUpErrorCode};
use crate::models::structsHandler::ZipCodeOptional;
use actix_web::{patch, web, HttpResponse};
use actix_web_lab::middleware::from_fn;
use sqlx::{MySql, QueryBuilder};

/// Update properties of a zip code
#[utoipa::path(
    context_path = "/zipcode",
    responses(
        (status = 200, description = "Update properties of a zip code", body = String),
        (status = 400, description = "Bad Request", body = PickUpError),
        (status = 401, description = "Unauthorized", body = PickUpError),
        (status = 500, description = "Internal Server Error", body = PickUpError)
    ),
    security(
      ("bearerAuth" = [])
   ),
    tag = "Zip Code"
)]
#[patch("/{id}", wrap = "from_fn(permissionLevelAdminMiddleware)")]
pub async fn patchZipCode(
    data: web::Data<AppState>,
    info: web::Json<ZipCodeOptional>,
    path: web::Path<String>,
) -> HttpResponse {
    let mut queryBuilder: QueryBuilder<'_, MySql> = QueryBuilder::new("UPDATE ZipCode SET ");
    let mut separated = queryBuilder.separated(", ");
    let mut countOfAdded: usize = 0;

    if info.Number.is_some() {
        separated.push("Number=");
        separated.push_bind_unseparated(info.Number.clone().unwrap());
        countOfAdded += 1;
    }

    if info.City.is_some() {
        separated.push("City=");
        separated.push_bind_unseparated(info.City.clone().unwrap());
        countOfAdded += 1;
    }

    if countOfAdded == 0 {
        let errorPickUp: PickUpError =
            PickUpError::newMessage(PickUpErrorCode::BadRequest, "No fields provided");
        return HttpResponse::BadRequest()
            .content_type("application/json")
            .json(errorPickUp);
    } else {
        queryBuilder.push(" WHERE Id=");
        queryBuilder.push_bind(path.into_inner());

        let query: Result<_, sqlx::Error> = queryBuilder.build().execute(&data.pool).await;

        match query {
            Err(e) => {
                return errorHandling::getHRFromErrorDatabase(e);
            }
            Ok(_) => {
                return HttpResponse::Ok().content_type("application/json").finish();
            }
        }
    }
}
