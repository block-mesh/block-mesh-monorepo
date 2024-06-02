use axum::response::Redirect;
use axum::{Extension, Form};
use axum_login::AuthSession;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::database::invite_code::create_invite_code::create_invite_code;
use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct EditInviteCodeForm {
    pub new_invite_code: String,
}

#[tracing::instrument(name = "edit_invite_code_post", skip(auth, pool))]
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
            "Invite code cannot contain spaces",
            "Invite code cannot contain spaces",
            "/edit_invite_code",
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
            "/edit_invite_code",
        ));
    };
    transaction.commit().await.map_err(Error::from)?;
    Ok(Redirect::to("/dashboard"))
}
