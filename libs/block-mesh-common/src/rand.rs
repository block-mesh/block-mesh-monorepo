use rand::Rng;

pub fn init_rand(min: i32, max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    let random_number: i32 = rng.gen_range(min..=max);
    random_number
}
