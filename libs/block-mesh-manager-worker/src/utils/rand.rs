use chrono::Datelike;
use chrono::Utc;
use rand::Rng;
use std::env;

pub fn rand_factor(limit: i32) -> i32 {
    let day = Utc::now().date_naive();
    let ordinal = day.ordinal() as i32;
    let f = ordinal
        % env::var("ORDINAL_FACTOR")
            .unwrap_or("3".to_string())
            .parse()
            .unwrap_or(3i32);
    let mut rng = rand::thread_rng();
    rng.gen_range(1..(limit * (1 + f)))
}
