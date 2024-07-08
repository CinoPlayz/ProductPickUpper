use actix_web::HttpResponse;

use crate::models::App::{PickUpError, PickUpErrorCode};

/// Get Http Response from sqlx::Error which can be Internal or Database Error
pub fn getHRFromErrorDatabase(error: sqlx::Error) -> HttpResponse {
    match error.as_database_error() {
        //e.g. If connection to database is lost
        None => {
            let errorPickUpInternal: PickUpError = error.into();
            return HttpResponse::InternalServerError()
                .content_type("application/json")
                .json(errorPickUpInternal);
        }
        Some(errorDatabase) => {
            let errorPickUp: PickUpError = errorDatabase.into();
            return HttpResponse::BadRequest().content_type("application/json").json(errorPickUp);
        }
    }
}

/// Get Http Response from sqlx::Error which can be only Internal
pub fn getHRFromErrorInternal(error: sqlx::Error) -> HttpResponse {
    let errorPickUpInternal: PickUpError = error.into();
    return HttpResponse::InternalServerError()
        .content_type("application/json")
        .json(errorPickUpInternal);
}

/// Get Http Response from sqlx::Error which can be IncorectCredentials or Database Error
pub fn getHRFromErrorIncorectCredentials(error: sqlx::Error) -> HttpResponse {
    match error.as_database_error(){
        None => {
           return HttpResponse::BadRequest().content_type("application/json").json(PickUpError::new(PickUpErrorCode::IncorectCredentials));
        }
        Some(errorDatabase) => {
           let errorPickUpDatabase: PickUpError = errorDatabase.into();
           return HttpResponse::BadRequest().content_type("application/json").json(errorPickUpDatabase);
        }
     }
}
