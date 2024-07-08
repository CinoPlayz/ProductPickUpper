use chrono::NaiveDateTime;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

//Token
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema, sqlx::FromRow)]
pub struct Token {
    pub Id: String,
    pub Token: String,
    pub Type: i8,
    pub DeviceInfo: String,
    pub DateStart: NaiveDateTime,
    pub DateEnd: NaiveDateTime,
    pub DateCreated: NaiveDateTime,
    pub FK_User: String
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
