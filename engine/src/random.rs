use rand::prelude::*;

/// Generate a random unsigned int between 0 and max.
///
/// The generation is very simple, it just takes the current time and take the module from it.
/// It is not a good random generator but will be ok for simple application;
pub fn rand(min: i32, max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

/// Random boolean, like a coin toss.
pub fn flip() -> bool {
    random()
}