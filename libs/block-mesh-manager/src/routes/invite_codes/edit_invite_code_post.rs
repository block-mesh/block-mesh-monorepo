use crate::database::invite_code::create_invite_code::create_invite_code;
use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;
use crate::startup::application::AppState;
use axum::extract::State;
use axum::response::Redirect;
use axum::{Extension, Form};
use axum_login::AuthSession;
use block_mesh_common::interfaces::server_api::EditInviteCodeForm;
use sqlx::PgPool;
use std::sync::Arc;

#[tracing::instrument(name = "edit_invite_code_post", skip_all)]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthSession<Backend>>,
    Form(form): Form<EditInviteCodeForm>,
) -> Result<Redirect, Error> {
    let mut transaction = pool.begin().await.map_err(Error::from)?;
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
    transaction.commit().await.map_err(Error::from)?;
    let email = user.email.clone();
    state
        .invite_codes
        .insert(email, form.new_invite_code, None)
        .await;
    Ok(Redirect::to("/ui/dashboard"))
}
