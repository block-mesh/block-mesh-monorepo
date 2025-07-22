use rand::Rng;
use std::cmp::max;

pub fn rand_factor(limit: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..max(limit, 2))
}
