use crate::server::test_app::spawn_app;
use crate::server::test_helpers::create_random_password;
use block_mesh_common::interfaces::server_api::RegisterForm;
use fake::faker::internet::raw::*;
use fake::locales::EN;
use fake::Fake;

#[tokio::test]
async fn test_register_user() {
    let app = spawn_app().await;
    let email: String = SafeEmail(EN).fake();
    let password: String = create_random_password();
    app.register_post(&RegisterForm {
        email,
        password: password.clone(),
        password_confirm: password.clone(),
        invite_code: Some("123".to_string()),
        cftoken: Option::from("test".to_string()),
    })
    .await
    .unwrap();
}
