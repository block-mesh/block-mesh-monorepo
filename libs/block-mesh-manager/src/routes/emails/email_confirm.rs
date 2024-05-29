use crate::database::nonce::get_nonce_by_user_id::get_nonce_by_user_id;
use crate::database::user::update_verified_email::update_verified_email;
use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;
use axum::extract::Query;
use axum::response::Redirect;
use axum::Extension;
use axum_login::AuthSession;
use block_mesh_common::interface::ConfirmEmailRequest;
use sqlx::PgPool;

#[tracing::instrument(name = "email_confirm", skip(auth, pool, query))]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthSession<Backend>>,
    Query(query): Query<ConfirmEmailRequest>,
) -> Result<Redirect, Error> {
    let mut transaction = pool.begin().await.map_err(Error::from)?;
    let user = auth.user.ok_or(Error::UserNotFound)?;
    let nonce = get_nonce_by_user_id(&mut transaction, &user.id).await?;
    return match nonce {
        None => Ok(Error::redirect(
            500,
            "Didn't find token".to_string(),
            "Please contact our support".to_string(),
        )),
        Some(nonce) => {
            if *nonce.nonce.expose_secret() != query.token {
                Ok(Error::redirect(
                    500,
                    "Token mismatch".to_string(),
                    "Please contact our support".to_string(),
                ))
            } else {
                update_verified_email(&mut transaction, user.id, true)
                    .await
                    .map_err(Error::from)?;
                transaction.commit().await.map_err(Error::from)?;
                Ok(Redirect::to("/dashboard"))
            }
        }
    };
}
