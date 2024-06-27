use crate::shared::{auth::permissionLevelAdminMiddleware, errorHandling};
use crate::shared::password::getHashedPassword;
use crate::shared::random::getRandomStr;
use crate::shared::structs::structsApp::AppState;
use crate::shared::structs::structsHandler::UserCreate;
use actix_web::{post, web, HttpResponse};
use actix_web_lab::middleware::from_fn;

/// Create a user
#[utoipa::path(
    context_path = "/user",
    responses(
        (status = 201, description = "Created user", body = String),
        (status = 400, description = "Bad Request", body = PickUpError),
        (status = 401, description = "Unauthorized", body = PickUpError),
        (status = 500, description = "Internal Server Error", body = PickUpError)
    ),
    security(
        ("bearerAuth" = [])
    ),
    tag = "User"
)]
#[post("", wrap="from_fn(permissionLevelAdminMiddleware)")]
pub async fn postUser(data: web::Data<AppState>, info: web::Json<UserCreate>) -> HttpResponse {
    let hashedPassword = getHashedPassword(
        &info.Password,
        &data.pepper,
        &getRandomStr(64),
        &data.hashingParameters,
    )
    .unwrap();

    let query: Result<_, sqlx::Error> = sqlx::query!(
        "INSERT INTO User ( Username , Name, Surname, Password, FK_UserRole) VALUES(?, ?, ?, ?, ?)",
        info.Username,
        info.Name,
        info.Surname,
        hashedPassword,
        info.FK_UserRole
    )
    .execute(&data.pool)
    .await;

    match query {
        Err(e) => {
            return errorHandling::getHRFromErrorDatabase(e);
        }
        Ok(_) => {
            return HttpResponse::Created()
                .content_type("application/json")
                .finish();
        }
    }
}
