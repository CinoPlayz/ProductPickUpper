use actix_web::{ post, web, HttpResponse };
use actix_web_httpauth::extractors::bearer::BearerAuth;
use sha256::digest;
use crate::shared::errorHandling;
use crate::shared::random::generateToken;
use crate::models::App::{ AppState, PickUpError, PickUpErrorCode };
use crate::models::Token::{ Token, TokenOnly };

#[utoipa::path(
    context_path = "/",
    responses(
        (status = 201, description = "Created access token", body = TokenOnly),
        (status = 400, description = "Bad Request", body = PickUpError),
        (status = 500, description = "Internal Server Error", body = PickUpError)
    ),
    security(
      ("bearerAuth" = [])
   ),
    tag = "Token"
)]
#[post("access")]
pub async fn access(data: web::Data<AppState>, auth: Option<BearerAuth>) -> HttpResponse {
    //Checks if refresh token is included in the request
    let Some(auth) = auth else {
        return HttpResponse::BadRequest()
            .content_type("application/json")
            .json(PickUpError::new(PickUpErrorCode::BadRequest));
    };

    let tokenHashed = digest(auth.token());

    let query = sqlx
        ::query_as!(
            Token,
            "SELECT * FROM Token WHERE Type=1 AND Token=? AND DateEnd > NOW()",
            tokenHashed
        )
        .fetch_one(&data.pool).await;

    match query {
        Err(e) => {
            if
                e.to_string() ==
                "no rows returned by a query that expected to return at least one row"
            {
                return HttpResponse::Unauthorized()
                    .content_type("application/json")
                    .json(PickUpError::new(PickUpErrorCode::Unauthorized));
            } else {
                return HttpResponse::InternalServerError()
                    .content_type("application/json")
                    .json(
                        PickUpError::newMessage(
                            PickUpErrorCode::InternalServerError,
                            &e.to_string()
                        )
                    );
            }
        }
        Ok(gottenToken) => {
            let tokenGenerated = generateToken();

            let queryInsertToken: Result<_, sqlx::Error> = sqlx
                ::query!(
                    "INSERT INTO Token (Token, Type, DeviceInfo, DateStart, DateEnd, FK_User) VALUES(?, 0, ?, NOW(), DATE_ADD(NOW(), INTERVAL 3600 SECOND), ?)",
                    &tokenGenerated.SHA256ofToken,
                    &gottenToken.DeviceInfo,
                    &gottenToken.FK_User
                )
                .execute(&data.pool).await;

            match queryInsertToken {
                Err(e) => {
                    return errorHandling::getHRFromErrorDatabase(e);
                }
                Ok(_) => {
                    let tokenOnly = TokenOnly {
                        Token: tokenGenerated.Token,
                    };
                    return HttpResponse::Created().content_type("application/json").json(tokenOnly);
                }
            }
        }
    }
}
