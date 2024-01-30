use chrono::NaiveDateTime;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

//UserRoles
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserRole {
    pub Id: String,
    pub Role: String,
    pub Description: String,
}

//User
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
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