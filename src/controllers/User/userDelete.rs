use crate::shared::{auth::permissionLevelAdminMiddleware, errorHandling};
use crate::models::structsApp::AppState;
use actix_web::{delete, web, HttpResponse};
use actix_web_lab::middleware::from_fn;

/// Delete a user
#[utoipa::path(
    context_path = "/user",
    responses(
        (status = 200, description = "Deleted user", body = String),
        (status = 400, description = "Bad Request", body = PickUpError),
        (status = 401, description = "Unauthorized", body = PickUpError),
        (status = 500, description = "Internal Server Error", body = PickUpError)
    ),
    security(
      ("bearerAuth" = [])
   ),
    tag = "User"
)]
#[delete("/{id}", wrap="from_fn(permissionLevelAdminMiddleware)")]
pub async fn deleteUser(data: web::Data<AppState>, path: web::Path<String>) -> HttpResponse {
    let query: Result<_, sqlx::Error> =
        sqlx::query!("DELETE FROM User WHERE Id=?", path.into_inner())
            .execute(&data.pool)
            .await;

    match query {
        Err(e) => {
            return errorHandling::getHRFromErrorDatabase(e);
        }
        Ok(_) => {
            return HttpResponse::Ok().content_type("application/json").finish();
        }
    }
}
