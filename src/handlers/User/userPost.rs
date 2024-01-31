use actix_web::{ post, web, HttpResponse };
use crate::shared::random::getRandomStr;
use crate::shared::structs::structsApp::{AppState, PickUpError};
use crate::shared::structs::structsHandler::UserCreate;
use crate::shared::password::getHashedPassword;

#[utoipa::path(
   context_path = "/",
   responses(
       (status = 200, description = "Inserted user", body = String),
       (status = 400, description = "Bad Request", body = PickUpError)
   )
)]
#[post("user")]
pub async fn postUser(data: web::Data<AppState>, info: web::Json<UserCreate>) -> HttpResponse {
   let hashedPassword = getHashedPassword(
      &info.Password,
      &data.pepper,
      &getRandomStr(64)
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
      Ok(_) => {return HttpResponse::Ok().content_type("application/json").json("")}, 
   }

}
