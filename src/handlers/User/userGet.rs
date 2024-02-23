use actix_web::{ get, web, HttpResponse };
use crate::shared::auth::getPermissionLevelHttp;
use crate::shared::structs::structsApp::{AppState, PermissionLevel, PickUpError};
use crate::shared::structs::structsHandler::User;
use crate::shared::structs::structsApp::PickUpErrorCode;
use actix_web_httpauth::extractors::bearer::BearerAuth;

/// Get all users
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
  tag="User" 
)]
#[get("user")]
pub async fn getAllUsers(data: web::Data<AppState>, auth: BearerAuth) -> HttpResponse {
   let token = auth.token();

   match getPermissionLevelHttp(token, &data.pool).await {
      Err(e) => {
         return e;
      },
      Ok(userPermissionLevel) => {
         if userPermissionLevel != PermissionLevel::Admin{
            HttpResponse::Unauthorized().content_type("application/json").json(PickUpError::new(PickUpErrorCode::Unauthorized))
         }
         else{
            let users = sqlx::query_as!(User, "SELECT * FROM User")
            .fetch_all(&data.pool).await.unwrap();
     
            HttpResponse::Ok()
             .content_type("application/json")
             .json(&users)
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
  tag="User"
)]
#[get("user/{id}")]
pub async fn getUserById(data: web::Data<AppState>, auth: BearerAuth, path: web::Path<String>) -> HttpResponse {
   let token = auth.token();

   match getPermissionLevelHttp(token, &data.pool).await {
      Err(e) => {
         return e;
      },
      Ok(userPermissionLevel) => {
         if userPermissionLevel != PermissionLevel::Admin{
            HttpResponse::Unauthorized().content_type("application/json").json(PickUpError::new(PickUpErrorCode::Unauthorized))
         }
         else{
            let uuid = path.into_inner();

            let users = sqlx::query_as!(User, "SELECT * FROM User WHERE Id=?", uuid)
            .fetch_all(&data.pool).await.unwrap();
     
            HttpResponse::Ok()
             .content_type("application/json")
             .json(&users)
         }


         
      }
   }  
   
}