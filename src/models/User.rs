use chrono::NaiveDateTime;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

//UserRoles
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema, sqlx::FromRow)]
pub struct UserRole {
    pub UserRoleId: String,
    pub PermissionLevel: i8,
    pub Role: String,
    pub Description: Option<String>,
}

//User
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema, sqlx::FromRow)]
pub struct User {
    pub UserId: String,
    pub Username: String,
    pub Name: String,
    pub Surname: String,
    pub Password: String,
    pub DateCreated: NaiveDateTime,
    #[sqlx(flatten)]
    pub UserRole: UserRole
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
pub struct UserOptional {
    pub Username: Option<String>,
    pub Name: Option<String>,
    pub Surname: Option<String>,
    pub Password: Option<String>,
    pub FK_UserRole: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct UserLogin {
    pub Username: String,
    pub Password: String,
    pub Active: u32, //How long should token be active in seconds
    pub DeviceInfo: String
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserCredentials {
    pub Id: String,
    pub Username: String,
    pub Password: String    
}