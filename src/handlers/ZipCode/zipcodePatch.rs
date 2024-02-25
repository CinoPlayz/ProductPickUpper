use actix_web::{ patch, web, HttpResponse };
use actix_web_httpauth::extractors::bearer::BearerAuth;
use sqlx::{ MySql, QueryBuilder };
use crate::shared::auth::getPermissionLevelHttp;
use crate::shared::errorHandling;
use crate::shared::structs::structsApp::{ AppState, PermissionLevel, PickUpError, PickUpErrorCode };
use crate::shared::structs::structsHandler::ZipCodeOptional;

/// Update properties of a zip code
#[utoipa::path(
    context_path = "/",
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
#[patch("zipcode/{id}")]
pub async fn patchZipCode(
    data: web::Data<AppState>,
    info: web::Json<ZipCodeOptional>,
    auth: BearerAuth,
    path: web::Path<String>
) -> HttpResponse {
    let token = auth.token();

    match getPermissionLevelHttp(token, &data.pool).await {
        Err(e) => {
            return e;
        }
        Ok(userPermissionLevel) => {
            if userPermissionLevel < PermissionLevel::Supervisor {
                HttpResponse::Unauthorized()
                    .content_type("application/json")
                    .json(PickUpError::new(PickUpErrorCode::Unauthorized))
            } else {
                let mut queryBuilder: QueryBuilder<'_, MySql> = QueryBuilder::new(
                    "UPDATE ZipCode SET "
                );
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
                    let errorPickUp: PickUpError = PickUpError::newMessage(
                        PickUpErrorCode::BadRequest,
                        "No fields provided"
                    );
                    return HttpResponse::BadRequest()
                        .content_type("application/json")
                        .json(errorPickUp);
                } else {
                    queryBuilder.push(" WHERE Id=");
                    queryBuilder.push_bind(path.into_inner());

                    let query: Result<_, sqlx::Error> = queryBuilder
                        .build()
                        .execute(&data.pool).await;

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
        }
    }
}
