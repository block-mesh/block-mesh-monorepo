use crate::server::test_app::spawn_app;
use block_mesh_common::routes_enum::RoutesEnum;
use reqwest::StatusCode;
use serde_json::{json, Value};
use sqlx::types::Json;
use uuid::Uuid;

async fn insert_user(
    app: &crate::server::test_app::TestApp,
    user_id: Uuid,
    email: &str,
    wallet_address: Option<&str>,
) {
    sqlx::query(
        r#"
        INSERT INTO users (id, email, password, wallet_address, created_at)
        VALUES ($1, $2, $3, $4, now())
        "#,
    )
    .bind(user_id)
    .bind(email)
    .bind("password-hash")
    .bind(wallet_address)
    .execute(&app.db_pool)
    .await
    .unwrap();
}

async fn insert_archived_user_snapshot(
    app: &crate::server::test_app::TestApp,
    record_id: Uuid,
    operation: &str,
    new_values: Option<Value>,
    old_values: Option<Value>,
) {
    sqlx::query(
        r#"
        INSERT INTO archives (
            id,
            table_name,
            record_type,
            record_id,
            operation,
            new_values,
            old_values,
            most_recent,
            created_at
        )
        VALUES ($1, 'users', 'User', $2, $3, $4, $5, TRUE, now())
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(record_id)
    .bind(operation)
    .bind(new_values.map(Json))
    .bind(old_values.map(Json))
    .execute(&app.db_pool)
    .await
    .unwrap();
}

fn archived_user_snapshot(user_id: Uuid, email: &str, wallet_address: Option<&str>) -> Value {
    json!({
        "id": user_id.to_string(),
        "email": email,
        "password": "archived-password-hash",
        "wallet_address": wallet_address,
        "created_at": "2026-01-01T00:00:00Z",
        "role": "user",
        "verified_email": false,
        "proof_of_humanity": false,
        "extension_activated": false,
        "extension_activated_sent": false,
        "wallet_connected_sent": false,
        "email_confirmed_sent": false,
        "snag_email_reward_pending": false,
        "snag_email_reward_consumed": false
    })
}

#[tokio::test]
async fn test_restore_user_from_archive_restores_deleted_snapshot_from_old_values() {
    let app = spawn_app().await;
    let user_id = Uuid::new_v4();
    let email = format!("restore-{}@example.com", Uuid::new_v4());

    insert_archived_user_snapshot(
        &app,
        user_id,
        "DELETE",
        None,
        Some(archived_user_snapshot(user_id, &email, None)),
    )
    .await;

    let response = app
        .client
        .post(format!(
            "{}/api{}",
            app.address,
            RoutesEnum::Api_RestoreUserFromArchive
        ))
        .json(&json!({
            "email": email,
            "team_api_key": "tmp"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    let payload: Value = response.json().await.unwrap();
    assert_eq!(payload["restored"], true);
    assert_eq!(payload["user_id"], user_id.to_string());

    let restored_email: String = sqlx::query_scalar("SELECT email FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(&app.db_pool)
        .await
        .unwrap();
    assert_eq!(restored_email, email);
}

#[tokio::test]
async fn test_restore_user_from_archive_returns_wallet_conflict_details() {
    let app = spawn_app().await;
    let existing_user_id = Uuid::new_v4();
    let archived_user_id = Uuid::new_v4();
    let archived_email = format!("restore-wallet-{}@example.com", Uuid::new_v4());
    let wallet_address = "wallet-conflict-address";

    insert_user(
        &app,
        existing_user_id,
        "existing-wallet-owner@example.com",
        Some(wallet_address),
    )
    .await;
    insert_archived_user_snapshot(
        &app,
        archived_user_id,
        "DELETE",
        None,
        Some(archived_user_snapshot(
            archived_user_id,
            &archived_email,
            Some(wallet_address),
        )),
    )
    .await;

    let response = app
        .client
        .post(format!(
            "{}/api{}",
            app.address,
            RoutesEnum::Api_RestoreUserFromArchive
        ))
        .json(&json!({
            "email": archived_email,
            "team_api_key": "tmp"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CONFLICT);
    let payload: Value = response.json().await.unwrap();
    assert_eq!(payload["restored"], false);
    assert_eq!(payload["conflict_field"], "wallet_address");
    assert_eq!(payload["conflict_value"], wallet_address);
    assert_eq!(payload["conflicting_user_id"], existing_user_id.to_string());
    assert!(payload["message"]
        .as_str()
        .unwrap_or_default()
        .contains("wallet_address"));

    let restored_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE id = $1")
        .bind(archived_user_id)
        .fetch_one(&app.db_pool)
        .await
        .unwrap();
    assert_eq!(restored_count, 0);
}

#[tokio::test]
async fn test_restore_user_from_archive_accepts_trimmed_code_key() {
    let app = spawn_app().await;
    let user_id = Uuid::new_v4();
    let email = format!("restore-code-{}@example.com", Uuid::new_v4());

    insert_archived_user_snapshot(
        &app,
        user_id,
        "DELETE",
        None,
        Some(archived_user_snapshot(user_id, &email, None)),
    )
    .await;

    let response = app
        .client
        .post(format!(
            "{}/api{}",
            app.address,
            RoutesEnum::Api_RestoreUserFromArchive
        ))
        .json(&json!({
            "email": email,
            "code": " tmp "
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
}
