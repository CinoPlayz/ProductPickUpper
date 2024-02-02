use chrono::NaiveDateTime;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

//UserRoles
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserRole {
    pub Id: String,
    pub PermissionLevel: i8,
    pub Role: String,
    pub Description: Option<String>,
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

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct UserCreate {
    pub Username: String,
    pub Name: String,
    pub Surname: String,
    pub Password: String,
    pub FK_UserRole: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct UserLogin {
    pub Username: String,
    pub Password: String,
    pub Active: u32, //How long should token be active in seconds
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserCredentials {
    pub Id: String,
    pub Username: String,
    pub Password: String    
}

//Token
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct Token {
    pub Id: String,
    pub Token: String,
    pub DateStart: NaiveDateTime,
    pub DateEnd: NaiveDateTime,
    pub DateCreated: NaiveDateTime,
    pub FK_User: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct TokenCreate {
    pub Token: String,
    pub DateStart: NaiveDateTime,
    pub DateEnd: NaiveDateTime,
    pub FK_User: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct TokenOnly {
    pub Token: String
}
