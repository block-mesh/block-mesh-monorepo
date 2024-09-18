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
    if form.new_invite_code.contains(' ') {
        return Ok(Error::redirect(
            400,
            "Invalid Invite",
            "Invite code cannot contain spaces",
            "/ui/dashboard",
        ));
    } else if !form.new_invite_code.chars().all(char::is_alphanumeric) {
        return Ok(Error::redirect(
            400,
            "Invalid Invite",
            "Invite code cannot contain special characters",
            "/ui/dashboard",
        ));
    }
    if create_invite_code(&mut transaction, user.id, form.new_invite_code)
        .await
        .is_err()
    {
        return Ok(Error::redirect(
            500,
            "Failed to create invite code",
            "Failed to create invite code, please try a different invite code",
            "/ui/dashboard",
        ));
    };
    transaction.commit().await.map_err(Error::from)?;
    Ok(Redirect::to("/ui/dashboard"))
}
