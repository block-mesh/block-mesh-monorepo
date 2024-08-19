use crate::server::test_app::spawn_app;
use block_mesh_common::interfaces::server_api::{RegisterForm, RegisterResponse};
use fake::faker::internet::raw::*;
use fake::locales::EN;
use fake::Fake;

#[tokio::test]
async fn test_register_user() {
    let app = spawn_app().await;
    let email: String = SafeEmail(EN).fake();
    let password: String = Password(EN, 8..20).fake();
    let response: RegisterResponse = app
        .register_post(&RegisterForm {
            email,
            password: password.clone(),
            password_confirm: password.clone(),
            invite_code: "123".to_string(),
        })
        .await;

    assert!(response.error.is_none());
    assert_eq!(200, response.status_code);
}
