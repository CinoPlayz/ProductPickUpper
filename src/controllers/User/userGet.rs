use crate::shared::{auth::permissionLevelAdminMiddleware, errorHandling};
use crate::models::structsApp::AppState;
use crate::models::structsHandler::User;
use actix_web::{get, web, HttpResponse};
use actix_web_lab::middleware::from_fn;

/// Get all users
#[utoipa::path(
    context_path = "/user",
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
#[get("", wrap="from_fn(permissionLevelAdminMiddleware)")]
pub async fn getAllUsers(data: web::Data<AppState>) -> HttpResponse {
    let query = sqlx::query_as::<_, User>(
        "SELECT u.Id AS 'UserId', Username, Name, Surname, Password, DateCreated, ur.Id AS 'UserRoleId', PermissionLevel, Role, Description  FROM User u INNER JOIN UserRole ur ON u.FK_UserRole=ur.Id"
    )
    .fetch_all(&data.pool).await;

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

/// Get user by Id
#[utoipa::path(
    context_path = "/user",
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
#[get("/{id}", wrap="from_fn(permissionLevelAdminMiddleware)")]
pub async fn getUserById(data: web::Data<AppState>, path: web::Path<String>) -> HttpResponse {
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
        Ok(users) => HttpResponse::Ok()
            .content_type("application/json")
            .json(&users),
    }
}
