use actix_web::{ get, web, HttpResponse };
use crate::shared::auth::authenticateHttp;
use crate::shared::errorHandling;
use crate::shared::structs::structsApp::AppState;
use crate::shared::structs::structsHandler::ZipCode;
use actix_web_httpauth::extractors::bearer::BearerAuth;

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
#[get("zipcode")]
pub async fn getAllZipCodes(data: web::Data<AppState>, auth: BearerAuth) -> HttpResponse {
    let token = auth.token();

    match authenticateHttp(token, &data.pool).await {
        Some(e) => {
            return e;
        }
        None => {
                let query: Result<Vec<ZipCode>, sqlx::Error> = sqlx::query_as!(ZipCode, "SELECT Id AS 'ZipCodeId', Number, City FROM ZipCode")
                .fetch_all(&data.pool).await;    

                match query {
                    //e.g. If connection to database is lost
                    Err(e) => {
                        return errorHandling::getHRFromErrorInternal(e);
                    }
                    Ok(users) => {
                        HttpResponse::Ok().content_type("application/json").json(&users)
                    }
                }
            
        }
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
#[get("zipcode/{id}")]
pub async fn getZipCodeById(
    data: web::Data<AppState>,
    auth: BearerAuth,
    path: web::Path<String>
) -> HttpResponse {
    let token = auth.token();

    match authenticateHttp(token, &data.pool).await {
        Some(e) => {
            return e;
        }
        None => {
            let uuid = path.into_inner();

            let query: Result<Vec<ZipCode>, sqlx::Error> = sqlx::query_as!(ZipCode, "SELECT Id AS 'ZipCodeId', Number, City FROM ZipCode WHERE Id=?", uuid)
            .fetch_all(&data.pool).await;

            match query {
                //e.g. If connection to database is lost
                Err(e) => {
                    return errorHandling::getHRFromErrorInternal(e);  
                }
                Ok(users) => {
                    HttpResponse::Ok().content_type("application/json").json(&users)
                }
            }
            
        }
    }
}
