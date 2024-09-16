use crate::database::aggregate::get_or_create_aggregate_by_user_and_name::get_or_create_aggregate_by_user_and_name_pool;
use crate::database::aggregate::update_aggregate::update_aggregate_pool;
use crate::domain::aggregate::AggregateName;
use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;
use crate::routes::twitter::context::{Oauth2Ctx, Oauth2CtxPg};
use axum::response::Redirect;
use axum::Extension;
use axum_login::AuthSession;
use block_mesh_common::constants::BLOCKMESH_SERVER_UUID_ENVAR;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::Mutex;
use twitter_v2::authorization::Scope;
use twitter_v2::oauth2::PkceCodeChallenge;
use uuid::Uuid;

pub async fn login(
    Extension(pool): Extension<PgPool>,
    Extension(ctx): Extension<Arc<Mutex<Oauth2Ctx>>>,
    Extension(auth): Extension<AuthSession<Backend>>,
) -> anyhow::Result<Redirect, Error> {
    let user = auth.user.ok_or(Error::UserNotFound)?;
    let ctx = ctx.lock().await;
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

    let id = Uuid::parse_str(std::env::var(BLOCKMESH_SERVER_UUID_ENVAR).unwrap().as_str()).unwrap();
    let twitter_agg =
        get_or_create_aggregate_by_user_and_name_pool(&pool, AggregateName::Twitter, &id).await?;
    let pg = Oauth2CtxPg {
        verifier: Some(verifier),
        state: Some(state),
        token: None,
        user_nonce: Some(user.nonce),
        user_id: Some(user.id),
    };

    update_aggregate_pool(&pool, &twitter_agg.id, &serde_json::to_value(&pg).unwrap()).await?;
    Ok(Redirect::to(url.as_ref()))
}
