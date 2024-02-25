use actix_web::{ get, web, HttpResponse };
use crate::shared::auth::getPermissionLevelHttp;
use crate::shared::errorHandling;
use crate::shared::structs::structsApp::{ AppState, PermissionLevel, PickUpError };
use crate::shared::structs::structsHandler::User;
use crate::shared::structs::structsApp::PickUpErrorCode;
use actix_web_httpauth::extractors::bearer::BearerAuth;

/// Get all users
#[utoipa::path(
    context_path = "/",
    responses(
        (status = 200, description = "Returns all users", body = Vec<User>),
        (status = 401, description = "Unauthorized", body = PickUpError),
        (status = 500, description = "Internal Server Error", body = PickUpError)
    ),
    security(
        ("bearerAuth" = [])
    ),
    tag = "User"
)]
#[get("user")]
pub async fn getAllUsers(data: web::Data<AppState>, auth: BearerAuth) -> HttpResponse {
    let token = auth.token();

    match getPermissionLevelHttp(token, &data.pool).await {
        Err(e) => {
            return e;
        }
        Ok(userPermissionLevel) => {
            if userPermissionLevel != PermissionLevel::Admin {
                HttpResponse::Unauthorized()
                    .content_type("application/json")
                    .json(PickUpError::new(PickUpErrorCode::Unauthorized))
            } else {
                let query = sqlx::query_as::<_, User>(
                        "SELECT u.Id AS 'UserId', Username, Name, Surname, Password, DateCreated, ur.Id AS 'UserRoleId', PermissionLevel, Role, Description  FROM User u INNER JOIN UserRole ur ON u.FK_UserRole=ur.Id"
                    )
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
}

/// Get users by Id
#[utoipa::path(
    context_path = "/",
    responses(
        (status = 200, description = "Returns all users", body = User),
        (status = 401, description = "Unauthorized", body = PickUpError),
        (status = 500, description = "Internal Server Error", body = PickUpError)
    ),
    security(
        ("bearerAuth" = [])
    ),
    tag = "User"
)]
#[get("user/{id}")]
pub async fn getUserById(
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
            if userPermissionLevel != PermissionLevel::Admin {
                HttpResponse::Unauthorized()
                    .content_type("application/json")
                    .json(PickUpError::new(PickUpErrorCode::Unauthorized))
            } else {
                let uuid = path.into_inner();

                let query = sqlx::query_as::<_, User>(
                        "SELECT u.Id AS 'UserId', Username, Name, Surname, Password, DateCreated, ur.ID AS 'UserRoleId', PermissionLevel, Role, Description FROM User u INNER JOIN UserRole ur ON u.FK_UserRole=ur.Id WHERE u.Id=?"
                    )
                    .bind(uuid)
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
}
