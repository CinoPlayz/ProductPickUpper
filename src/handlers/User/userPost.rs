use actix_web::{ post, web, HttpResponse };
use actix_web_httpauth::extractors::bearer::BearerAuth;
use crate::shared::auth::getPermissionLevelHttp;
use crate::shared::random::getRandomStr;
use crate::shared::structs::structsApp::{AppState, PermissionLevel, PickUpError, PickUpErrorCode};
use crate::shared::structs::structsHandler::UserCreate;
use crate::shared::password::getHashedPassword;

/// Create a user
#[utoipa::path(
   context_path = "/",
   responses(
       (status = 201, description = "Created user", body = String),
       (status = 400, description = "Bad Request", body = PickUpError),
       (status = 401, description = "Unauthorized", body = PickUpError),
       (status = 500, description = "Internal Server Error", body = PickUpError)
   ),
   security(
      ("bearerAuth" = [])
  ),
  tag="User"
)]
#[post("user")]
pub async fn postUser(data: web::Data<AppState>, info: web::Json<UserCreate>, auth: BearerAuth) -> HttpResponse {
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
            let hashedPassword = getHashedPassword(
               &info.Password,
               &data.pepper,
               &getRandomStr(64),
               &data.hashingParameters
            ).unwrap();
         
            let query: Result<_, sqlx::Error> = sqlx::query!(
               "INSERT INTO User ( Username , Name, Surname, Password, FK_UserRole) VALUES(?, ?, ?, ?, ?)",
               info.Username,
               info.Name,
               info.Surname,
               hashedPassword,
               info.FK_UserRole
            )
            .execute(&data.pool).await;
         
            match query{
               Err(e)=> {
                  let errorPickUp: PickUpError = e.as_database_error().unwrap().into();
                  return HttpResponse::BadRequest().content_type("application/json").json(errorPickUp)
               }
               Ok(_) => {return HttpResponse::Created().content_type("application/json").finish()}, 
            }
         }

      }
   }


   

}
