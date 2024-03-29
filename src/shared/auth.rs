use actix_web::HttpResponse;
use sha256::digest;
use sqlx::{ MySql, Pool };

use super::structs::structsApp::PermissionLevel;
use super::structs::structsApp::PickUpError;
use super::structs::structsApp::PickUpErrorCode;
use crate::shared::structs::structsApp::PermissionLevelStruct;

//TODO: Make this middleware
//TODO: Make MVC structure
//TODO: Update to latest
async fn authenticate(token: &str, pool: &Pool<MySql>) -> Result<bool, PickUpError> {
    let tokenHashed = digest(token);

    let query: Result<_, sqlx::Error> = sqlx
        ::query!("SELECT Id FROM Token WHERE Token=? AND DateEnd > NOW()", tokenHashed)
        .fetch_one(pool).await;

    match query {
        Err(e) => {
            if
                e.to_string() ==
                "no rows returned by a query that expected to return at least one row"
            {
                return Err(PickUpError::new(PickUpErrorCode::Unauthorized));
            } else {
                return Err(PickUpError {
                    Code: PickUpErrorCode::InternalServerError,
                    Message: e.to_string(),
                });
            }
        }
        Ok(_) => {
            return Ok(true);
        }
    }
}

/// Returns HttpResponse if error occurs while authenticating (No return means authenticated successfully)
pub async fn authenticateHttp(token: &str, pool: &Pool<MySql>) -> Option<HttpResponse> {
    match authenticate(token, pool).await {
        Err(e) => {
            if e.Code == PickUpErrorCode::Unauthorized {
                return Some(HttpResponse::Unauthorized().content_type("application/json").json(e));
            } else {
                return Some(
                    HttpResponse::InternalServerError().content_type("application/json").json(e)
                );
            }
        }
        Ok(_) => {
            return None;
        }
    }
}


async fn getPermissionLevel(token: &str, pool: &Pool<MySql>) -> Result<PermissionLevel, PickUpError> {
    let tokenHashed = digest(token);

    let query: Result<PermissionLevelStruct, sqlx::Error> = sqlx
        ::query_as!(PermissionLevelStruct, "SELECT ur.PermissionLevel FROM UserRole ur INNER JOIN User u ON ur.Id=u.FK_UserRole INNER JOIN Token t ON u.Id=t.FK_User WHERE Token=? AND t.DateEnd > NOW()", tokenHashed)
        .fetch_one(pool).await;

    match query {
        Err(e) => {
            if
                e.to_string() ==
                "no rows returned by a query that expected to return at least one row"
            {
                return Err(PickUpError::new(PickUpErrorCode::Unauthorized));
            } else {
                //If not connection to database can be made
                return Err(PickUpError {
                    Code: PickUpErrorCode::InternalServerError,
                    Message: e.to_string(),
                });
            }
        }
        Ok(permissionLevel) => {
            return match permissionLevel.PermissionLevel  {
                1 => Ok(PermissionLevel::Supervisor),
                2 => Ok(PermissionLevel::Admin),
                _ => Ok(PermissionLevel::User)
            }
        }
    }
}

/// Authenticates and get PermissionLevel or HttpResponse if error
pub async fn getPermissionLevelHttp(token: &str, pool: &Pool<MySql>) -> Result<PermissionLevel, HttpResponse> {
    match getPermissionLevel(token, pool).await {
        Err(e) => {
            if e.Code == PickUpErrorCode::Unauthorized {
                return Err(HttpResponse::Unauthorized().content_type("application/json").json(e));
            } else {
                return Err(
                    HttpResponse::InternalServerError().content_type("application/json").json(e)
                );
            }
        }
        Ok(permissionLevel) => {
            return Ok(permissionLevel);
        }
    }
}
