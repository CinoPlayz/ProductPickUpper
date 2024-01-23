use chrono::NaiveDateTime;
use sqlx::{MySql, Pool};
use serde::Deserialize;
use serde::Serialize;


pub struct AppState {
    pub version: String,
    pub pool: Pool<MySql>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User{
    pub Id: String,
    pub Username: String,
    pub Name: String,
    pub Surname: String,
    pub Password: String,
    pub DateCreated: NaiveDateTime,
    pub FK_UserRole: String
}