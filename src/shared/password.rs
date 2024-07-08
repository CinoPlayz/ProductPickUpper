use argon2::{ self, Config, Variant, Version };
use sqlx::{ MySql, Pool };

use crate::models::structsApp::{ HashingParameters, PickUpError };
use crate::models::structsHandler::UserRole;
use crate::shared::random::getRandomStr;

pub fn getHashedPassword(
    password: &str,
    papper: &str,
    salt: &str,
    hashingParameters: &HashingParameters
) -> Result<String, argon2::Error> {
    let config = Config {
        variant: Variant::Argon2id,
        version: Version::Version13,
        mem_cost: hashingParameters.mem_cost,
        time_cost: hashingParameters.time_cost,
        lanes: hashingParameters.lanes,
        secret: papper.as_bytes(),
        ad: &[],
        hash_length: 290,
    };

    return argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config);
}

pub fn isPasswordCorrect(password: &str, hash: &str, papper: &str) -> Result<bool, argon2::Error> {
    return argon2::verify_encoded_ext(hash, password.as_bytes(), papper.as_bytes(), &[]);
}

pub async fn createRoot(
    pool: &Pool<MySql>,
    papper: &str,
    hashingParameters: &HashingParameters
) -> Result<(), PickUpError> {
    let userRoleHighestPer = sqlx
        ::query_as!(UserRole, "SELECT Id AS 'UserRoleId', PermissionLevel, Role, Description FROM UserRole ORDER BY PermissionLevel DESC LIMIT 1")
        .fetch_all(pool).await
        .unwrap();

    let query: Result<_, sqlx::Error> = sqlx
        ::query!(
            "INSERT INTO User ( Username , Name, Surname, Password, FK_UserRole) VALUES(?, ?, ?, ?, ?)",
            "root",
            "",
            "",
            getHashedPassword("admin", papper, &getRandomStr(64), hashingParameters).unwrap(),
            userRoleHighestPer[0].UserRoleId
        )
        .execute(pool).await;

    match query {
        Err(e) => {            
            let errorPickUp: PickUpError = e.as_database_error().unwrap().into();
            return Err(errorPickUp);
        }
        Ok(_) => {
            return Ok(());
        }
    }
}
