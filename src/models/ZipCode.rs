use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;

//ZipCode
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema, sqlx::FromRow)]
pub struct ZipCode {
    pub ZipCodeId: String,
    pub Number: i32,
    pub City: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct ZipCodeCreate {
    pub Number: i32,
    pub City: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct ZipCodeOptional {
    pub Number: Option<i32>,
    pub City: Option<String>,
}
