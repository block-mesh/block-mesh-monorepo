use crate::routes::twitter::context::Oauth2Ctx;
use axum::response::{IntoResponse, Redirect};
use axum::Extension;
use std::sync::Arc;
use tokio::sync::Mutex;
use twitter_v2::authorization::Scope;
use twitter_v2::oauth2::PkceCodeChallenge;

pub async fn login(Extension(ctx): Extension<Arc<Mutex<Oauth2Ctx>>>) -> impl IntoResponse {
    let mut ctx = ctx.lock().await;
    // create challenge
    let (challenge, verifier) = PkceCodeChallenge::new_random_sha256();
    // create authorization url
    let (url, state) = ctx
        .client
        .auth_url(challenge, [Scope::FollowsRead, Scope::UsersRead]);
    // set context for reference in callback
    ctx.verifier = Some(verifier);
    ctx.state = Some(state);
    // redirect user
    Redirect::to(&url.to_string())
}
