use crate::database::nonce::get_nonce_by_nonce::get_nonce_by_nonce_pool;
use crate::database::user::get_user_by_id::get_user_opt_by_id;
use crate::database::user::update_verified_email::update_verified_email;
use crate::errors::error::Error;
use crate::notification::notification_redirect::NotificationRedirect;
use axum::extract::Query;
use axum::response::Redirect;
use axum::Extension;
use block_mesh_common::interfaces::server_api::ConfirmEmailRequest;
use block_mesh_common::routes_enum::RoutesEnum;
use sqlx::PgPool;

#[tracing::instrument(name = "email_confirm", skip(pool, query))]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Query(query): Query<ConfirmEmailRequest>,
) -> Result<Redirect, Error> {
    let mut transaction = pool.begin().await.map_err(Error::from)?;
    let nonce = get_nonce_by_nonce_pool(&pool, &query.token).await?;
    match nonce {
        None => Ok(Error::redirect(
            500,
            "Didn't find token",
            "Please contact our support",
            RoutesEnum::Static_UnAuth_Root.to_string().as_str(),
        )),
        Some(nonce) => {
            if *nonce.nonce.expose_secret() != query.token {
                Ok(Error::redirect(
                    500,
                    "Token mismatch",
                    "Please contact our support",
                    RoutesEnum::Static_UnAuth_Root.to_string().as_str(),
                ))
            } else {
                let user = get_user_opt_by_id(&mut transaction, &nonce.user_id)
                    .await
                    .map_err(Error::from)?;
                if user.is_none() {
                    return Ok(Error::redirect(
                        500,
                        "User not found",
                        "Please contact our support",
                        RoutesEnum::Static_UnAuth_Root.to_string().as_str(),
                    ));
                }
                update_verified_email(&mut transaction, user.unwrap().id, true)
                    .await
                    .map_err(Error::from)?;
                transaction.commit().await.map_err(Error::from)?;
                Ok(NotificationRedirect::redirect(
                    "Please Login",
                    "You email confirmed, please login into your account",
                    RoutesEnum::Static_UnAuth_Login.to_string().as_str(),
                ))
            }
        }
    }
}
