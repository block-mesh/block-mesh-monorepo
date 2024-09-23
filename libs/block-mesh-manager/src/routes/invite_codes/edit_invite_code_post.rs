use crate::database::invite_code::create_invite_code::create_invite_code;
use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;
use axum::response::Redirect;
use axum::{Extension, Form};
use axum_login::AuthSession;
use block_mesh_common::interfaces::server_api::EditInviteCodeForm;
use sqlx::PgPool;

pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthSession<Backend>>,
    Form(form): Form<EditInviteCodeForm>,
) -> Result<Redirect, Error> {
    let mut transaction = pool.begin().await.map_err(Error::from)?;
    let user = auth.user.ok_or(Error::UserNotFound)?;
    if !form.new_invite_code.chars().all(char::is_alphanumeric) {
        return Err(Error::InternalServer);
    }
    if create_invite_code(&mut transaction, user.id, form.new_invite_code)
        .await
        .is_err()
    {
        return Err(Error::InternalServer);
    };
    transaction.commit().await.map_err(Error::from)?;
    Ok(Redirect::to("/ui/dashboard"))
}
