use crate::server::test_app::{spawn_app, spawn_app_with_snag_base_url, TestApp};
use crate::server::test_helpers::create_random_password;
use block_mesh_common::interfaces::server_api::{ConfirmEmailRequest, RegisterForm};
use fake::faker::internet::raw::*;
use fake::locales::EN;
use fake::Fake;
use serde_json::Value;
use tokio::time::{sleep, Duration};
use uuid::Uuid;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn register_user(app: &TestApp) -> (String, String) {
    let email: String = SafeEmail(EN).fake();
    let password: String = create_random_password();
    app.register_post(&RegisterForm {
        email: email.clone(),
        password: password.clone(),
        password_confirm: password.clone(),
        invite_code: Some("123".to_string()),
        cftoken: Option::from("test".to_string()),
    })
    .await
    .unwrap();
    (email, password)
}

async fn wait_for_snag_email_reward_consumed(app: &TestApp, email: &str) {
    for _ in 0..50 {
        if app.snag_email_reward_consumed(email).await.unwrap() {
            return;
        }
        sleep(Duration::from_millis(50)).await;
    }
    panic!("timed out waiting for snag_email_reward_consumed");
}

async fn wait_for_request_count(
    server: &MockServer,
    expected_count: usize,
) -> Vec<wiremock::Request> {
    for _ in 0..50 {
        let requests = server.received_requests().await.unwrap();
        if requests.len() >= expected_count {
            return requests;
        }
        sleep(Duration::from_millis(50)).await;
    }
    panic!("timed out waiting for mock server requests");
}

fn request_body_json(request: &wiremock::Request) -> Value {
    serde_json::from_slice(&request.body).unwrap()
}

#[tokio::test]
async fn test_register_user_marks_snag_reward_pending() {
    let app = spawn_app().await;
    let (email, _) = register_user(&app).await;
    assert!(!app.verified_email(&email).await.unwrap());
    assert!(app.snag_email_reward_pending(&email).await.unwrap());
    assert!(!app.snag_email_reward_consumed(&email).await.unwrap());
}

#[tokio::test]
async fn test_register_user_creates_placeholder_snag_user_when_email_not_found() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/users"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": [],
            "hasNextPage": false
        })))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/api/users/metadatas"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "ok"
        })))
        .mount(&server)
        .await;

    let app = spawn_app_with_snag_base_url(server.uri()).await;
    let (email, _) = register_user(&app).await;

    let requests = wait_for_request_count(&server, 2).await;
    assert_eq!(requests.len(), 2);
    assert!(app.snag_email_reward_pending(&email).await.unwrap());
    assert!(!app.snag_email_reward_consumed(&email).await.unwrap());

    let metadata_request = requests
        .iter()
        .find(|request| request.url.path() == "/api/users/metadatas")
        .unwrap();
    let metadata_body = request_body_json(metadata_request);
    assert_eq!(metadata_body["emailAddress"], email);
    assert_eq!(metadata_body["displayName"], email);
    assert!(!metadata_body["walletAddress"]
        .as_str()
        .unwrap_or_default()
        .is_empty());
}

#[tokio::test]
async fn test_register_user_rewards_verified_snag_email_immediately() {
    let server = MockServer::start().await;
    let snag_user_id = Uuid::from_u128(77);
    Mock::given(method("GET"))
        .and(path("/api/users"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": [
                {
                    "id": snag_user_id,
                    "emailVerifiedAt": "2026-03-25T00:00:00Z"
                }
            ],
            "hasNextPage": false
        })))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path(
            "/api/loyalty/rules/test-email-registered-rule/complete",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "message": "Completion request added to queue",
            "data": {}
        })))
        .mount(&server)
        .await;

    let app = spawn_app_with_snag_base_url(server.uri()).await;
    let (email, _) = register_user(&app).await;

    wait_for_snag_email_reward_consumed(&app, &email).await;
    assert!(!app.snag_email_reward_pending(&email).await.unwrap());
    assert!(app.snag_email_reward_consumed(&email).await.unwrap());

    let requests = wait_for_request_count(&server, 2).await;
    let complete_request = requests
        .iter()
        .find(|request| {
            request.url.path() == "/api/loyalty/rules/test-email-registered-rule/complete"
        })
        .unwrap();
    let complete_body = request_body_json(complete_request);
    assert_eq!(complete_body["userId"], snag_user_id.to_string());
    assert!(complete_body.get("walletAddress").is_none());
}

#[tokio::test]
async fn test_email_confirm_api_keeps_success_when_snag_reward_retry_fails() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/users"))
        .respond_with(ResponseTemplate::new(500).set_body_string("boom"))
        .mount(&server)
        .await;

    let app = spawn_app_with_snag_base_url(server.uri()).await;
    let (email, _) = register_user(&app).await;
    let nonce = app.get_nonce(&email).await.unwrap();

    let response = app
        .confirm_email_api(&ConfirmEmailRequest {
            token: nonce,
            email: email.clone(),
        })
        .await
        .unwrap();
    assert!(response.success);

    let requests = wait_for_request_count(&server, 2).await;
    assert_eq!(requests.len(), 2);
    assert!(app.verified_email(&email).await.unwrap());
    assert!(app.snag_email_reward_pending(&email).await.unwrap());
    assert!(!app.snag_email_reward_consumed(&email).await.unwrap());
}
