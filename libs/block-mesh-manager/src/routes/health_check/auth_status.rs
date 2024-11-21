use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;
use crate::startup::application::AppState;
use axum::extract::State;
use axum::{Extension, Json};
use axum_login::AuthSession;
use block_mesh_common::interfaces::server_api::AuthStatusResponse;
use block_mesh_manager_database_domain::domain::get_user_opt_by_id::get_user_opt_by_id;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use std::sync::Arc;

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthSession<Backend>>,
) -> Result<Json<AuthStatusResponse>, Error> {
    let user = auth.user.ok_or(Error::UserNotFound)?;
    let mut transaction = create_txn(&state.follower_pool).await?;
    let db_user = get_user_opt_by_id(&mut transaction, &user.id)
        .await
        .map_err(Error::from)?;
    commit_txn(transaction).await?;
    if let Some(db_user) = db_user {
        return Ok(Json(AuthStatusResponse {
            email: Some(user.email.to_ascii_lowercase()),
            status_code: 200,
            logged_in: true,
            wallet_address: db_user.wallet_address,
        }));
    }
    Ok(Json(AuthStatusResponse {
        email: None,
        status_code: 404,
        logged_in: false,
        wallet_address: None,
    }))
}
