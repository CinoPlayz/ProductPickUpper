use sqlx::error::DatabaseError;
use sqlx::{ MySql, Pool };
use serde::Deserialize;
use serde::Serialize;
use regex::Regex;
use utoipa::OpenApi;
use crate::handlers::User::userGet;

use super::structsHandler::User;


pub struct AppState {
    pub version: String,
    pub pepper: String,
    pub pool: Pool<MySql>,
}

#[derive(OpenApi)]
#[openapi(
    info(title = "Product Pick Upper"),
)]
#[openapi(paths(userGet::getAllUsers), components(schemas(User)))]
pub struct ApiDoc;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PickUpError {
    pub Code: u32,
    pub Message: String,
}

impl From<&dyn DatabaseError> for PickUpError {
    fn from(e: &dyn DatabaseError) -> Self {
        if e.is_check_violation() {
            Self {
                Code: 1,
                Message: e.message().to_string(),
            }
        } else if e.is_foreign_key_violation() {
            let regex = Regex::new(r"FOREIGN KEY \((.*)\) REFERENCES").unwrap();
            match regex.captures(e.message()).map(|caps| caps.extract()) {
                None => {
                    Self {
                        Code: 2,
                        Message: format!("Foreign key constraint fails at unknown column"),
                    }
                }
                Some((_, [column])) => {
                    Self {
                        Code: 2,
                        Message: format!(
                            "Foreign key constraint fails at {} column",
                            column.replace("`", "")
                        ),
                    }
                }
            }
        } else if e.is_unique_violation() {
            Self {
                Code: 3,
                Message: e.message().to_string(),
            }
        } else {
            Self {
                Code: 0,
                Message: e.message().to_string(),
            }
        }
    }
}
