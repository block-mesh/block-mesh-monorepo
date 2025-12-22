use rand::Rng;
use secret::Secret;
use serde::{Deserialize, Serialize};
use std::iter;
use time::OffsetDateTime;
use uuid::Uuid;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct Nonce {
    pub id: Uuid,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    pub user_id: Uuid,
    #[serde(skip)]
    pub nonce: Secret<String>,
}

impl Nonce {
    pub fn generate_nonce(len: usize) -> String {
        let mut rng = rand::thread_rng();
        let one_char = || CHARSET[rng.gen_range(0..CHARSET.len())] as char;
        iter::repeat_with(one_char).take(len).collect()
    }
}
