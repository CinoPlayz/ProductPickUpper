use sqlx::error::DatabaseError;
use sqlx::{ MySql, Pool };
use serde::Deserialize;
use serde::Serialize;
use regex::Regex;
use utoipa::openapi::security::{ HttpAuthScheme, HttpBuilder, SecurityScheme };
use utoipa::{ Modify, OpenApi, ToSchema };
use crate::handlers::User::{ userGet, userPost, userPatch };
use crate::handlers::Token::login;
use derive_more::Display;

use super::structsHandler::{ TokenOnly, User, UserCreate, UserLogin, UserOptional };

pub struct AppState {
    pub version: String,
    pub pepper: String,
    pub pool: Pool<MySql>,
    pub createRoot: bool,
    pub hashingParameters: HashingParameters,
}

pub struct HashingParameters {
    pub mem_cost: u32,
    pub time_cost: u32,
    pub lanes: u32,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema, Display)]
pub enum PickUpErrorCode {
    #[display(fmt = "Other")]
    Other = 0,

    #[display(fmt = "Check")]
    Check = 1,

    #[display(fmt = "ForeignKey")]
    ForeignKey = 2,

    #[display(fmt = "Unique")]
    Unique = 3,

    #[display(fmt = "Hashing")]
    Hashing = 4,

    #[display(fmt = "IncorectCredentials")]
    IncorectCredentials = 5,

    #[display(fmt = "Unauthorized")]
    Unauthorized = 6,

    #[display(fmt = "InternalServerError")]
    InternalServerError = 7,

    #[display(fmt = "BadRequest")]
    BadRequest = 8,

    #[display(fmt = "Timeout")]
    Timeout = 9,
}

//Default to string
impl PickUpErrorCode {
    pub fn to_string(&self) -> String {
        match self {
            PickUpErrorCode::Other => format!("Other type of error"),
            PickUpErrorCode::Check => format!("Check constraint has failed"),
            PickUpErrorCode::ForeignKey => format!("Foreign key constraint has failed"),
            PickUpErrorCode::Unique => format!("Unique constraint has failed"),
            PickUpErrorCode::Hashing => format!("Hashing has failed"),
            PickUpErrorCode::IncorectCredentials => format!("Password or username is incorrect"),
            PickUpErrorCode::Unauthorized => format!("Token is insufficient"),
            PickUpErrorCode::InternalServerError => format!("Internal server error"),
            PickUpErrorCode::BadRequest => format!("Bad request error"),
            PickUpErrorCode::Timeout => format!("Timeout error"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema, Display)]
#[display(fmt = "Code: {} Message: {}", Code, Message)]
pub struct PickUpError {
    pub Code: PickUpErrorCode,
    pub Message: String,
}

impl PickUpError {
    pub fn new(pickupErrorCode: PickUpErrorCode) -> PickUpError {
        return match pickupErrorCode {
            PickUpErrorCode::IncorectCredentials =>
                PickUpError {
                    Code: PickUpErrorCode::IncorectCredentials,
                    Message: PickUpErrorCode::IncorectCredentials.to_string(),
                },
            PickUpErrorCode::Unauthorized =>
                PickUpError {
                    Code: PickUpErrorCode::Unauthorized,
                    Message: PickUpErrorCode::Unauthorized.to_string(),
                },

            _ =>
                PickUpError {
                    Code: PickUpErrorCode::Other,
                    Message: PickUpErrorCode::Other.to_string(),
                },
        };
    }

    pub fn newMessage(pickupErrorCode: PickUpErrorCode, message: &str) -> PickUpError {
        return match pickupErrorCode {
            PickUpErrorCode::IncorectCredentials =>
                PickUpError {
                    Code: PickUpErrorCode::IncorectCredentials,
                    Message: message.to_string(),
                },
            PickUpErrorCode::Unauthorized =>
                PickUpError {
                    Code: PickUpErrorCode::Unauthorized,
                    Message:message.to_string(),
                },

            PickUpErrorCode::Check =>
                PickUpError {
                    Code: PickUpErrorCode::Check,
                    Message: message.to_string(),
                },

            PickUpErrorCode::BadRequest =>
                PickUpError {
                    Code: PickUpErrorCode::BadRequest,
                    Message: message.to_string(),
                },

            PickUpErrorCode::ForeignKey =>
                PickUpError {
                    Code: PickUpErrorCode::ForeignKey,
                    Message: message.to_string(),
                },

            PickUpErrorCode::Hashing =>
                PickUpError {
                    Code: PickUpErrorCode::Hashing,
                    Message: message.to_string(),
                },

            PickUpErrorCode::InternalServerError =>
                PickUpError {
                    Code: PickUpErrorCode::InternalServerError,
                    Message: message.to_string(),
                },

            PickUpErrorCode::Timeout =>
                PickUpError {
                    Code: PickUpErrorCode::Timeout,
                    Message: message.to_string(),
                },

            PickUpErrorCode::Unique =>
                PickUpError {
                    Code: PickUpErrorCode::Unique,
                    Message: message.to_string(),
                },                

            _ =>
                PickUpError {
                    Code: PickUpErrorCode::Other,
                    Message: message.to_string(),
                },
        };
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
            Code: PickUpErrorCode::Hashing,
            Message: format!("Error while hashing password {}", e.to_string()),
        }
    }
}

pub struct GeneratedToken {
    pub Token: String,
    pub SHA256ofToken: String,
}

#[derive(Debug, PartialEq)]
pub enum PermissionLevel {
    User = 0,
    Supervisor = 1,
    Admin = 2,
}

pub struct PermissionLevelStruct {
    pub PermissionLevel: i8,
}

#[derive(OpenApi)]
#[openapi(
    info(title = "Product Pick Upper"),
    paths(userGet::getAllUsers, userGet::getUserById, userPost::postUser, userPatch::patchUser,  login::login),
    components(schemas(User, UserCreate, UserLogin, UserOptional, TokenOnly, PickUpError, PickUpErrorCode, )),
    tags(
        (name = "User", description = "User management endpoints"),
        (name = "Token", description = "Token management endpoints")
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components: &mut utoipa::openapi::Components = openapi.components.as_mut().unwrap(); // we can unwrap safely since there already is components registered.        
        components.add_security_scheme(
            "bearerAuth",
            SecurityScheme::Http(
                HttpBuilder::new().scheme(HttpAuthScheme::Bearer).bearer_format("JWT").build()
            )
        )
    }
}

