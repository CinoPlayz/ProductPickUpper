use actix_web::{ delete, web, HttpResponse };
use actix_web_httpauth::extractors::bearer::BearerAuth;
use crate::shared::auth::getPermissionLevelHttp;
use crate::shared::errorHandling;
use crate::shared::structs::structsApp::{ AppState, PermissionLevel, PickUpError, PickUpErrorCode };

/// Delete zip code
#[utoipa::path(
    context_path = "/",
    responses(
        (status = 200, description = "Deleted zip code", body = String),
        (status = 400, description = "Bad Request", body = PickUpError),
        (status = 401, description = "Unauthorized", body = PickUpError),
        (status = 500, description = "Internal Server Error", body = PickUpError)
    ),
    security(
      ("bearerAuth" = [])
   ),
    tag = "Zip Code"
)]
#[delete("zipcode/{id}")]
pub async fn deleteZipCode(
    data: web::Data<AppState>,
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
                let query: Result<_, sqlx::Error> = sqlx::query!("DELETE FROM ZipCode WHERE Id=?", path.into_inner())
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
