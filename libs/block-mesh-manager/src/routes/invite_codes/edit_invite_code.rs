use crate::database::invite_code::get_user_latest_invite_code::get_user_latest_invite_code;
use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;
use askama::Template;
use askama_axum::IntoResponse;
use axum::Extension;
use axum_login::AuthSession;
use sqlx::PgPool;

#[derive(Template)]
#[template(path = "invite_codes/edit_invite_code.html")]
struct EditInviteCodeTemplate {
    pub current_invite_code: String,
}

#[tracing::instrument(name = "edit_invite_code", skip(auth))]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthSession<Backend>>,
) -> Result<impl IntoResponse, Error> {
    let mut transaction = pool.begin().await.map_err(Error::from)?;
    let user = auth.user.ok_or(Error::UserNotFound)?;
    let user_invite_code = get_user_latest_invite_code(&mut transaction, user.id)
        .await
        .map_err(Error::from)?;
    transaction.commit().await.map_err(Error::from)?;
    Ok(EditInviteCodeTemplate {
        current_invite_code: user_invite_code.invite_code,
    })
}
