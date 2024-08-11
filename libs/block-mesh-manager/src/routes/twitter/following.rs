use crate::routes::twitter::context::Oauth2Ctx;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use block_mesh_common::constants::BLOCKMESH_TWITTER_USER_ID;
use http::StatusCode;
use std::sync::Arc;
use tokio::sync::Mutex;
use twitter_v2::TwitterApi;

pub async fn following(Extension(ctx): Extension<Arc<Mutex<Oauth2Ctx>>>) -> impl IntoResponse {
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
    let following = api
        .get_user_followers(BLOCKMESH_TWITTER_USER_ID)
        .send()
        .await;
    tracing::info!("following => {:#?}", following);
    // get tweet by id
    Ok::<_, (StatusCode, String)>(Json(following.unwrap().into_data()))
}
