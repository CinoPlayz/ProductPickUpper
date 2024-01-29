use argon2::{self, Config, Variant, Version};

pub fn getHashedPassword(password: &str, papper: &str, salt: &str) -> Result<String, argon2::Error>{
    let config = Config {
        variant: Variant::Argon2id,
        version: Version::Version13,
        mem_cost: 65536,
        time_cost: 10,
        lanes: 4,
        secret: papper.as_bytes(),
        ad: &[],
        hash_length: 294
    };
    
   return argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config);
}

pub fn isPasswordCorrect(password: &str, hash: &str, papper: &str) -> Result<bool, argon2::Error>{
    return argon2::verify_encoded_ext(hash, password.as_bytes(), papper.as_bytes(), &[])
}