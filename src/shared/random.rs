use rand::distributions::{Alphanumeric, DistString};
use sha256::digest;

use crate::models::App::GeneratedToken;

pub fn getRandomStr(length: usize) -> String {
    return Alphanumeric.sample_string(&mut rand::thread_rng(), length);
}

pub fn generateToken() -> GeneratedToken {
    let token = getRandomStr(64);

    return GeneratedToken{
        SHA256ofToken: digest(&token),
        Token: token        
    };
}