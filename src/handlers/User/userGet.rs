use actix_web::{ get, web, HttpResponse };
use crate::shared::structs::{AppState, User};

#[get("user")]
pub async fn getAllUsers(data: web::Data<AppState>) -> HttpResponse {
   
   let users = sqlx::query_as!(User, "SELECT * FROM User")
    .fetch_all(&data.pool).await.unwrap();

       HttpResponse::Ok()
        .content_type("application/json")
        .json(&users)
}