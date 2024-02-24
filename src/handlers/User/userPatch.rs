use actix_web::{ patch, web, HttpResponse };
use actix_web_httpauth::extractors::bearer::BearerAuth;
use sqlx::{ MySql, QueryBuilder };
use crate::shared::auth::getPermissionLevelHttp;
use crate::shared::errorHandling;
use crate::shared::random::getRandomStr;
use crate::shared::structs::structsApp::{ AppState, PermissionLevel, PickUpError, PickUpErrorCode };
use crate::shared::structs::structsHandler::UserOptional;
use crate::shared::password::getHashedPassword;

/// Update properties of a user
#[utoipa::path(
    context_path = "/",
    responses(
        (status = 200, description = "Update properties of a user", body = String),
        (status = 400, description = "Bad Request", body = PickUpError),
        (status = 401, description = "Unauthorized", body = PickUpError),
        (status = 500, description = "Internal Server Error", body = PickUpError)
    ),
    security(
      ("bearerAuth" = [])
   ),
    tag = "User"
)]
#[patch("user/{id}")]
pub async fn patchUser(
    data: web::Data<AppState>,
    info: web::Json<UserOptional>,
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
                let mut queryBuilder: QueryBuilder<'_, MySql> = QueryBuilder::new(
                    "UPDATE User SET "
                );
                let mut separated = queryBuilder.separated(", ");
                let mut countOfAdded: usize = 0;

                if info.Username.is_some() {
                    separated.push("Username=");
                    separated.push_bind_unseparated(info.Username.clone().unwrap());
                    countOfAdded += 1;
                }

                if info.Name.is_some() {
                    separated.push("Name=");
                    separated.push_bind_unseparated(info.Name.clone().unwrap());
                    countOfAdded += 1;
                }

                if info.Surname.is_some() {
                    separated.push("Surname=");
                    separated.push_bind_unseparated(info.Surname.clone().unwrap());
                    countOfAdded += 1;
                }

                if info.FK_UserRole.is_some() {
                    separated.push("FK_UserRole=");
                    separated.push_bind_unseparated(info.FK_UserRole.clone().unwrap());
                    countOfAdded += 1;
                }

                if info.Password.is_some() {
                    let hashedPassword = getHashedPassword(
                        &info.Password.clone().unwrap(),
                        &data.pepper,
                        &getRandomStr(64),
                        &data.hashingParameters
                    ).unwrap();

                    separated.push("Password=");
                    separated.push_bind_unseparated(hashedPassword);
                    countOfAdded += 1;
                }

                if countOfAdded == 0 {
                    let errorPickUp: PickUpError = PickUpError::newMessage(
                        PickUpErrorCode::BadRequest,
                        "No fields provided"
                    );
                    return HttpResponse::BadRequest()
                        .content_type("application/json")
                        .json(errorPickUp);
                } else {
                    queryBuilder.push(" WHERE Id=");
                    queryBuilder.push_bind(path.into_inner());

                    let query: Result<_, sqlx::Error> = queryBuilder
                        .build()
                        .execute(&data.pool).await;

                    match query {
                        Err(e) => {
                            return errorHandling::getHRFromErrorDatabase(e);                           
                        }
                        Ok(_) => {
                            return HttpResponse::Ok().content_type("application/json").finish();
                        }
                    }
                }
            }
        }
    }
}
