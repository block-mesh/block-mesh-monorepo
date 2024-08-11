use crate::routes::twitter::context::Oauth2Ctx;
use axum::extract::Query;
use axum::response::{IntoResponse, Redirect};
use axum::Extension;
use http::StatusCode;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;
use twitter_v2::oauth2::{AuthorizationCode, CsrfToken};

#[derive(Deserialize)]
pub struct CallbackParams {
    code: AuthorizationCode,
    state: CsrfToken,
}

pub async fn callback(
    Extension(ctx): Extension<Arc<Mutex<Oauth2Ctx>>>,
    Query(CallbackParams { code, state }): Query<CallbackParams>,
) -> impl IntoResponse {
    let (client, verifier) = {
        let mut ctx = ctx.lock().await;
        // get previous state from ctx (see login)
        let saved_state = ctx.state.take().ok_or_else(|| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "No previous state found".to_string(),
            )
        })?;
        // // check state returned to see if it matches, otherwise throw an error
        if state.secret() != saved_state.secret() {
            return Err((
                StatusCode::BAD_REQUEST,
                "Invalid state returned".to_string(),
            ));
        }
        // // get verifier from ctx
        let verifier = ctx.verifier.take().ok_or_else(|| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "No PKCE verifier found".to_string(),
            )
        })?;
        let client = ctx.client.clone();
        (client, verifier)
    };

    // request oauth2 token
    let token = client
        .request_token(code, verifier)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    // set context for use with twitter API
    tracing::info!(
        "TOKEN = {:#?} | {:#?}",
        token.access_token().secret(),
        token.refresh_token()
    );
    ctx.lock().await.token = Some(token);

    Ok(Redirect::to("/twitter/following"))
}
