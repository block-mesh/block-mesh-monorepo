use crate::server::test_app::spawn_app;
use crate::server::test_helpers::create_random_password;
use block_mesh_common::interfaces::server_api::{GetTokenRequest, RegisterForm};
use block_mesh_common::interfaces::ws_api::WsServerMessage;
use fake::faker::internet::raw::*;
use fake::locales::EN;
use fake::Fake;
use futures::{SinkExt, StreamExt, TryStreamExt};
use reqwest::Client;
use reqwest_websocket::{Message, RequestBuilderExt};

#[tokio::test]
async fn test_connect_to_ws() {
    let app = spawn_app().await;
    let email: String = SafeEmail(EN).fake();
    let password: String = create_random_password();
    app.register_post(&RegisterForm {
        email: email.clone(),
        password: password.clone(),
        password_confirm: password.clone(),
        invite_code: "123".to_string(),
    })
    .await
    .unwrap();
    let api_token = app
        .get_api_token(&GetTokenRequest {
            email: email.clone(),
            password: password.clone(),
        })
        .await
        .unwrap();
    assert!(api_token.api_token.is_some());
    let api_token = api_token.api_token.unwrap();
    let response = Client::default()
        .get(app.ws_address_with_auth(&email, &api_token))
        .upgrade() // Prepares the WebSocket upgrade.
        .send()
        .await
        .unwrap();
    // Turns the response into a WebSocket stream.
    let mut websocket = response.into_websocket().await.unwrap();
    // The WebSocket implements `Sink<Message>`.
    websocket.send(Message::Text("Ping".into())).await.unwrap();
}

#[tokio::test]
async fn test_send_msg_to_ws() {
    let app = spawn_app().await;
    let email: String = SafeEmail(EN).fake();
    let password: String = create_random_password();
    app.register_post(&RegisterForm {
        email: email.clone(),
        password: password.clone(),
        password_confirm: password.clone(),
        invite_code: "123".to_string(),
    })
    .await
    .unwrap();
    let api_token = app
        .get_api_token(&GetTokenRequest {
            email: email.clone(),
            password: password.clone(),
        })
        .await
        .unwrap();
    assert!(api_token.api_token.is_some());
    let api_token = api_token.api_token.unwrap();
    let response = Client::default()
        .get(app.ws_address_with_auth(&email, &api_token))
        .upgrade() // Prepares the WebSocket upgrade.
        .send()
        .await
        .unwrap();
    // Turns the response into a WebSocket stream.
    let mut websocket = response.into_websocket().await.unwrap();
    let (_tx, mut rx) = websocket.split();
    while let Some(message) = rx.try_next().await.unwrap() {
        if let Message::Text(text) = message {
            let msg = serde_json::from_str::<WsServerMessage>(&text).unwrap();
            match msg {
                WsServerMessage::RequestUptimeReport => break,
                _ => panic!("Unexpected"),
            }
        }
    }
}
