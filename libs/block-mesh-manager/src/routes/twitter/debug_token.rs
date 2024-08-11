use crate::routes::twitter::context::Oauth2Ctx;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use http::StatusCode;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn debug_token(Extension(ctx): Extension<Arc<Mutex<Oauth2Ctx>>>) -> impl IntoResponse {
    // get oauth token
    let oauth_token = ctx
        .lock()
        .await
        .token
        .as_ref()
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, "User not logged in!".to_string()))?
        .clone();
    // get underlying token
    Ok::<_, (StatusCode, String)>(Json(oauth_token))
}
