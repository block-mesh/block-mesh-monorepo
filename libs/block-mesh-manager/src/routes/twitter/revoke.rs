use crate::routes::twitter::context::Oauth2Ctx;
use axum::response::IntoResponse;
use axum::Extension;
use http::StatusCode;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn revoke(Extension(ctx): Extension<Arc<Mutex<Oauth2Ctx>>>) -> impl IntoResponse {
    // get oauth token
    let (oauth_token, oauth_client) = {
        let ctx = ctx.lock().await;
        let token = ctx
            .token
            .as_ref()
            .ok_or_else(|| (StatusCode::UNAUTHORIZED, "User not logged in!".to_string()))?
            .clone();
        let client = ctx.client.clone();
        (token, client)
    };
    // revoke token
    oauth_client
        .revoke_token(oauth_token.revokable_token())
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
    Ok::<_, (StatusCode, String)>("Token revoked!")
}
