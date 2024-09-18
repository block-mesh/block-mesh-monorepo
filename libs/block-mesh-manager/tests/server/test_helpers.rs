use rand::Rng;
use std::iter;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

pub fn create_random_password() -> String {
    let mut rng = rand::thread_rng();
    let one_char = || CHARSET[rng.gen_range(0..CHARSET.len())] as char;
    let pass: String = iter::repeat_with(one_char).take(10).collect();
    format!("{}{}", pass, "&")
}
