use crate::routes::twitter::context::Oauth2Ctx;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use http::StatusCode;
use std::sync::Arc;
use tokio::sync::Mutex;
use twitter_v2::TwitterApi;

pub async fn tweets(Extension(ctx): Extension<Arc<Mutex<Oauth2Ctx>>>) -> impl IntoResponse {
    tracing::info!("here 1");
    // get oauth token
    let (mut oauth_token, oauth_client) = {
        let ctx = ctx.lock().await;
        let token = ctx
            .token
            .as_ref()
            .ok_or_else(|| (StatusCode::UNAUTHORIZED, "User not logged in!".to_string()))?
            .clone();
        let client = ctx.client.clone();
        (token, client)
    };
    tracing::info!("here 2");
    // refresh oauth token if expired
    if oauth_client
        .refresh_token_if_expired(&mut oauth_token)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    {
        // save oauth token if refreshed
        ctx.lock().await.token = Some(oauth_token.clone());
    }
    tracing::info!("here 3");

    let api = TwitterApi::new(oauth_token);
    let me = api.get_users_me().send().await;
    tracing::info!("me => {:#?}", me);
    // get tweet by id
    let tweet = api
        .get_tweet(20)
        .send()
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
    Ok::<_, (StatusCode, String)>(Json(tweet.into_data()))
}
