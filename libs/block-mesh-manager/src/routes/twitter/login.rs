use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;
use crate::routes::twitter::context::Oauth2Ctx;
use axum::response::Redirect;
use axum::Extension;
use axum_login::AuthSession;
use std::sync::Arc;
use tokio::sync::Mutex;
use twitter_v2::authorization::Scope;
use twitter_v2::oauth2::PkceCodeChallenge;

pub async fn login(
    Extension(ctx): Extension<Arc<Mutex<Oauth2Ctx>>>,
    Extension(auth): Extension<AuthSession<Backend>>,
) -> anyhow::Result<Redirect, Error> {
    let user = auth.user.ok_or(Error::UserNotFound)?;

    let mut ctx = ctx.lock().await;
    // create challenge
    let (challenge, verifier) = PkceCodeChallenge::new_random_sha256();
    // create authorization url
    let (url, state) = ctx.client.auth_url(
        challenge,
        [
            Scope::TweetRead,
            Scope::TweetWrite,
            Scope::UsersRead,
            Scope::OfflineAccess,
        ],
    );
    // set context for reference in callback
    ctx.verifier = Some(verifier);
    ctx.state = Some(state);
    ctx.user_id = Some(user.id);
    ctx.user_nonce = Some(user.nonce);
    // redirect user
    Ok(Redirect::to(url.as_ref()))
}
