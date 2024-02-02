use sqlx::error::DatabaseError;
use sqlx::{ MySql, Pool };
use serde::Deserialize;
use serde::Serialize;
use regex::Regex;
use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};
use utoipa::{ Modify, OpenApi, ToSchema };
use crate::handlers::User::{userGet, userPost};
use crate::handlers::Token::login;

use super::structsHandler::{TokenOnly, User, UserCreate, UserLogin};

pub struct AppState {
    pub version: String,
    pub pepper: String,
    pub pool: Pool<MySql>,
    pub createRoot: bool,
    pub hashingParameters: HashingParameters
}

pub struct HashingParameters{
    pub mem_cost: u32,
    pub time_cost: u32,
    pub lanes: u32
}


#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum PickUpErrorCode {
    Other = 0,
    Check = 1,
    ForeignKey = 2,
    Unique = 3,
    HashingError = 4,
    IncorectCredentials = 5,
    Unauthorized = 6,
    InternalServerError = 7,
}


impl PickUpErrorCode  {
    pub fn to_string(&self) -> String {
        match self {
            PickUpErrorCode::IncorectCredentials => format!("Password or username is incorrect"),
            PickUpErrorCode::Unauthorized => format!("Token is insufficient"),
            _  => format!(""),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct PickUpError {
    pub Code: PickUpErrorCode,
    pub Message: String,
}

impl PickUpError {
    pub fn new(pickupErrorCode: PickUpErrorCode) -> PickUpError {
        return match pickupErrorCode {
            PickUpErrorCode::IncorectCredentials => PickUpError{
                Code:  PickUpErrorCode::IncorectCredentials,
                Message:  PickUpErrorCode::IncorectCredentials.to_string()
            },
            PickUpErrorCode::Unauthorized => PickUpError{
                Code:  PickUpErrorCode::Unauthorized,
                Message:  PickUpErrorCode::Unauthorized.to_string()
            },

            _ => PickUpError{
                Code:  PickUpErrorCode::Other,
                Message:  PickUpErrorCode::Other.to_string()
            }
        }
    }
}

impl From<&dyn DatabaseError> for PickUpError {
    fn from(e: &dyn DatabaseError) -> Self {
        if e.is_check_violation() {
            Self {
                Code: PickUpErrorCode::Check,
                Message: e.message().to_string(),
            }
        } else if e.is_foreign_key_violation() {
            let regex = Regex::new(r"FOREIGN KEY \((.*)\) REFERENCES").unwrap();
            match regex.captures(e.message()).map(|caps| caps.extract()) {
                None => {
                    Self {
                        Code: PickUpErrorCode::ForeignKey,
                        Message: format!("Foreign key constraint fails at unknown column"),
                    }
                }
                Some((_, [column])) => {
                    Self {
                        Code: PickUpErrorCode::ForeignKey,
                        Message: format!(
                            "Foreign key constraint fails at {} column",
                            column.replace("`", "")
                        ),
                    }
                }
            }
        } else if e.is_unique_violation() {
            Self {
                Code: PickUpErrorCode::Unique,
                Message: e.message().to_string(),
            }
        } else {
            Self {
                Code: PickUpErrorCode::Other,
                Message: e.message().to_string(),
            }
        }
    }
}


impl From<argon2::Error> for PickUpError {
    fn from(e: argon2::Error) -> Self {
            Self {
                Code: PickUpErrorCode::HashingError,
                Message: format!(
                    "Error while hashing password {}",
                    e.to_string()
                )
            }
       
    }
}

pub struct GeneratedToken{
    pub Token: String,
    pub SHA256ofToken: String,
}

#[derive(Debug, PartialEq)]
pub enum PermissionLevel {
    User = 0,
    Supervisor = 1,
    Admin = 2
}

pub struct PermissionLevelStruct {
    pub PermissionLevel: i8
}

#[derive(OpenApi)]
#[openapi(info(title = "Product Pick Upper"))]
#[openapi(paths(userGet::getAllUsers, userPost::postUser, login::login), components(schemas(User, UserCreate, UserLogin, TokenOnly, PickUpError, PickUpErrorCode)), modifiers(&SecurityAddon))]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components: &mut utoipa::openapi::Components = openapi.components.as_mut().unwrap(); // we can unwrap safely since there already is components registered.
        components.add_security_scheme(
            "bearerAuth",
            SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
        )
    }
}

