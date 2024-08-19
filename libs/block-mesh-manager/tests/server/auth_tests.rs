use crate::server::test_app::spawn_app;
use block_mesh_common::interfaces::server_api::{RegisterForm, RegisterResponse};
use fake::faker::internet::raw::*;
use fake::locales::EN;
use fake::Fake;
use rand::Rng;
use std::iter;
const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

pub fn create_random_password() -> String {
    let mut rng = rand::thread_rng();
    let one_char = || CHARSET[rng.gen_range(0..CHARSET.len())] as char;
    let pass: String = iter::repeat_with(one_char).take(10).collect();
    format!("{}{}", pass, "&")
}

#[tokio::test]
async fn test_register_user() {
    let app = spawn_app().await;
    let email: String = SafeEmail(EN).fake();
    let password: String = create_random_password();
    app.register_post(&RegisterForm {
        email,
        password: password.clone(),
        password_confirm: password.clone(),
        invite_code: "123".to_string(),
    })
    .await
    .unwrap();
}
