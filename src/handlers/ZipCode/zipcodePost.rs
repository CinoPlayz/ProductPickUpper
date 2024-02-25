use actix_web::{ post, web, HttpResponse };
use actix_web_httpauth::extractors::bearer::BearerAuth;
use crate::shared::auth::getPermissionLevelHttp;
use crate::shared::errorHandling;
use crate::shared::structs::structsApp::{ AppState, PermissionLevel, PickUpError, PickUpErrorCode };
use crate::shared::structs::structsHandler::ZipCodeCreate;

/// Create a zip code
#[utoipa::path(
    context_path = "/",
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
#[post("zipcode")]
pub async fn postZipCode(
    data: web::Data<AppState>,
    info: web::Json<ZipCodeCreate>,
    auth: BearerAuth
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
                let query: Result<_, sqlx::Error> = sqlx::query!(
                        "INSERT INTO ZipCode ( Number , City) VALUES(?, ?)",
                        info.Number,
                        info.City
                    )
                    .execute(&data.pool).await;

                match query {
                    Err(e) => {
                        return errorHandling::getHRFromErrorDatabase(e);  
                    }
                    Ok(_) => {
                        return HttpResponse::Created().content_type("application/json").finish();
                    }
                }
            }
        }
    }
}
