use crate::database::invite_code::create_invite_code::create_invite_code;
use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;
use crate::startup::application::AppState;
use axum::extract::State;
use axum::response::Redirect;
use axum::{Extension, Form};
use axum_login::AuthSession;
use block_mesh_common::interfaces::server_api::EditInviteCodeForm;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use sqlx::PgPool;
use std::sync::Arc;

#[tracing::instrument(name = "edit_invite_code_post", skip_all)]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthSession<Backend>>,
    Form(form): Form<EditInviteCodeForm>,
) -> Result<Redirect, Error> {
    let mut transaction = create_txn(&pool).await?;
    let user = auth.user.ok_or(Error::UserNotFound)?;
    if !form.new_invite_code.chars().all(char::is_alphanumeric) {
        return Err(Error::InternalServer);
    }
    if create_invite_code(&mut transaction, user.id, &form.new_invite_code)
        .await
        .is_err()
    {
        return Err(Error::InternalServer);
    };
    commit_txn(transaction).await?;
    state
        .invite_codes
        .insert(user.email.clone(), form.new_invite_code.clone(), None)
        .await;
    Ok(Redirect::to("/ui/dashboard"))
}
