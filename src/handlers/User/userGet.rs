use actix_web::{ get, web, HttpResponse };
use crate::shared::structs::structsApp::AppState;
use crate::shared::structs::structsHandler::User;
use actix_web_httpauth::extractors::bearer::BearerAuth;


#[utoipa::path(
   context_path = "/",
   responses(
       (status = 200, description = "Returns all users", body = User)
   )
)]
#[get("user")]
pub async fn getAllUsers(data: web::Data<AppState>, auth: BearerAuth) -> HttpResponse {
   let token = auth.token().to_string();
   
   let users = sqlx::query_as!(User, "SELECT * FROM User")
    .fetch_all(&data.pool).await.unwrap();

       HttpResponse::Ok()
        .content_type("application/json")
        .json(&users)
}