use actix_web::{ post, web, HttpResponse };
use crate::shared::random::generateToken;
use crate::shared::structs::structsApp::{ AppState, PickUpError, PickUpErrorCode };
use crate::shared::structs::structsHandler::{ TokenOnly, UserCredentials, UserLogin };
use crate::shared::password::isPasswordCorrect;

#[utoipa::path(
    context_path = "/",
    responses(
        (status = 201, description = "Created token", body = TokenOnly),
        (status = 400, description = "Bad Request", body = PickUpError),
        (status = 500, description = "Internal Server Error", body = PickUpError)
    )
)]
#[post("login")]
pub async fn login(data: web::Data<AppState>, info: web::Json<UserLogin>) -> HttpResponse {
    let queryUserCredetials:Result<UserCredentials, sqlx::Error> = sqlx
        ::query_as!(UserCredentials, "SELECT Id, Username, Password FROM User WHERE Username=?", info.Username)
        .fetch_one(&data.pool).await;

    match queryUserCredetials {
        Err(e) => {
            match e.as_database_error(){
               None => {
                  let errorPickUpDatabase= PickUpError{ Code: PickUpErrorCode::IncorectCredentials, Message: PickUpErrorCode::IncorectCredentials.to_string() };
                  return HttpResponse::BadRequest().content_type("application/json").json(errorPickUpDatabase);
               }
               Some(errorDatabase) => {
                  let errorPickUpDatabase: PickUpError = errorDatabase.into();
                  return HttpResponse::BadRequest().content_type("application/json").json(errorPickUpDatabase);
               }
            }
           
        }
        Ok(userCredentials) => {

            match isPasswordCorrect(&info.Password, &userCredentials.Password, &data.pepper){
               Err(e) => {
                  let errorPickUpHashing: PickUpError = e.into();
                  return HttpResponse::InternalServerError().content_type("application/json").json(errorPickUpHashing);
               },

               Ok(isPasswordCorrect) => {

                  if !isPasswordCorrect{
                     let errorPickUpCredentials: PickUpError = PickUpError{                     
                        Code: PickUpErrorCode::IncorectCredentials,
                        Message: PickUpErrorCode::IncorectCredentials.to_string()                     
                     };
                     return HttpResponse::BadRequest().content_type("application/json").json(errorPickUpCredentials);
                  }
                  else{
                     
                     let tokenGenerated = generateToken();                    

                     let queryInsertToken: Result<_, sqlx::Error> = sqlx::query!(
                        "INSERT INTO Token (Token, DateStart, DateEnd, FK_User) VALUES(?, NOW(), DATE_ADD(NOW(), INTERVAL ? SECOND), ?)",
                        &tokenGenerated.SHA256ofToken,
                        info.Active,                        
                        &userCredentials.Id
                     ).execute(
                           &data.pool
                     ).await;
                  
                     match queryInsertToken {
                           Err(e) => {
                              let errorPickUp: PickUpError = e.as_database_error().unwrap().into();
                              return HttpResponse::BadRequest().content_type("application/json").json(errorPickUp);
                           }
                           Ok(_) => {
                              let tokenOnly = TokenOnly{
                                 Token: tokenGenerated.Token
                              };
                              return HttpResponse::Created().content_type("application/json").json(tokenOnly);
                           }
                     }
                  }
               },
            }
            
         
            
        }
    }

}
