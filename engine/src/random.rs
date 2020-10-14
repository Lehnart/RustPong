use std::time::{SystemTime, UNIX_EPOCH};

/// Generate a random unsigned int between 0 and max.
///
/// The generation is very simple, it just takes the current time and take the module from it.
/// It is not a good random generator but will be ok for simple application;
pub fn rand(min:i32, max : i32) -> i32{
    let rand = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() % (max-min+1) as u128;
    rand as i32 + min
}

pub fn flip()->bool{
    if rand(0,1) == 1 {
        return true;
    }
    return false
}