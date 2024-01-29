use chrono::NaiveDateTime;
use sqlx::error::DatabaseError;
use sqlx::{ MySql, Pool };
use serde::Deserialize;
use serde::Serialize;
use regex::Regex;

pub struct AppState {
    pub version: String,
    pub pepper: String,
    pub pool: Pool<MySql>,
}

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

//UserRoles
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserRole {
    pub Id: String,
    pub Role: String,
    pub Description: String,
}

//User
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub Id: String,
    pub Username: String,
    pub Name: String,
    pub Surname: String,
    pub Password: String,
    pub DateCreated: NaiveDateTime,
    pub FK_UserRole: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserCreate {
    pub Username: String,
    pub Name: String,
    pub Surname: String,
    pub Password: String,
    pub FK_UserRole: String,
}
