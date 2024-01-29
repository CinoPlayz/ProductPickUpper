use rand::distributions::{Alphanumeric, DistString};

pub fn getRandomStr(length: usize) -> String {
    return Alphanumeric.sample_string(&mut rand::thread_rng(), length);
}