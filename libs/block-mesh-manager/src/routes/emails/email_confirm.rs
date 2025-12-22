use crate::database::nonce::get_nonce_by_nonce::get_nonce_by_nonce;
use crate::database::nonce::update_nonce::update_nonce;
use crate::database::spam_email::get_spam_emails::get_spam_emails_cache;
use crate::database::user::update_email::update_email;
use crate::database::user::update_verified_email::update_verified_email;
use crate::domain::spam_email::SpamEmail;
use crate::errors::error::Error;
use crate::middlewares::authentication::{del_from_cache, Backend};
use crate::notification::notification_redirect::NotificationRedirect;
use axum::extract::Query;
use axum::response::Redirect;
use axum::Extension;
use axum_login::AuthSession;
use block_mesh_common::interfaces::server_api::ConfirmEmailRequest;
use block_mesh_common::routes_enum::RoutesEnum;
use block_mesh_manager_database_domain::domain::get_user_opt_by_id::get_user_opt_by_id;
use sqlx::PgPool;

pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Extension(mut auth): Extension<AuthSession<Backend>>,
    Query(query): Query<ConfirmEmailRequest>,
) -> Result<Redirect, Error> {
    let mut transaction = match pool.begin().await {
        Ok(t) => t,
        Err(_) => {
            return Ok(Error::redirect(
                500,
                "Failed",
                "Failed retry in few seconds please",
                RoutesEnum::Static_UnAuth_Root.to_string().as_str(),
            ));
        }
    };
    let nonce = match get_nonce_by_nonce(&mut transaction, &query.token).await {
        Ok(o) => o,
        Err(_) => {
            return Ok(Error::redirect(
                400,
                "Bad Nonce",
                "Bad nonce",
                RoutesEnum::Static_UnAuth_Register.to_string().as_str(),
            ));
        }
    };
    let email = query.email.clone().to_ascii_lowercase();
    let spam_emails = get_spam_emails_cache().await;
    let email_domain = match email.split('@').next_back() {
        Some(d) => d.to_string(),
        None => {
            return Ok(Error::redirect(
                400,
                "Invalid email domain",
                "Please check if email you inserted is correct",
                RoutesEnum::Static_UnAuth_Register.to_string().as_str(),
            ));
        }
    };
    if SpamEmail::check_domains(&email_domain, spam_emails).is_err() {
        return Ok(Error::redirect(
            400,
            "Invalid email domain",
            "Please check if email you inserted is correct",
            RoutesEnum::Static_UnAuth_Register.to_string().as_str(),
        ));
    }

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
                let user = user.unwrap();
                match update_verified_email(&mut transaction, &user.id, true)
                    .await
                    .map_err(Error::from)
                {
                    Ok(_) => {}
                    Err(_) => {
                        return Ok(Error::redirect(
                            500,
                            "Failed to update",
                            "Failed to update verified email",
                            RoutesEnum::Static_UnAuth_Root.to_string().as_str(),
                        ));
                    }
                }
                if user.email != email {
                    match update_email(&mut transaction, &user.id, &email)
                        .await
                        .map_err(Error::from)
                    {
                        Ok(_) => {}
                        Err(_) => {
                            return Ok(Error::redirect(
                                500,
                                "Failed to update new email",
                                "Failed to update new email",
                                RoutesEnum::Static_UnAuth_Root.to_string().as_str(),
                            ));
                        }
                    }
                    auth.logout()
                        .await
                        .map_err(|e| Error::Auth(e.to_string()))?;
                    let key = Backend::authenticate_key_with_user_id(&user.id);
                    del_from_cache(&key).await;
                }
                match update_nonce(&mut transaction, user.id).await {
                    Ok(_) => {}
                    Err(_) => {
                        return Ok(Error::redirect(
                            500,
                            "Failed to update nonce",
                            "Failed to update nonce",
                            RoutesEnum::Static_UnAuth_Root.to_string().as_str(),
                        ));
                    }
                }
                match transaction.commit().await {
                    Ok(_) => {}
                    Err(_) => {
                        return Ok(Error::redirect(
                            500,
                            "Failed to save",
                            "Failed to save, please retry again in few seconds",
                            RoutesEnum::Static_UnAuth_Root.to_string().as_str(),
                        ));
                    }
                }
                Ok(NotificationRedirect::redirect(
                    "Please Login",
                    "You email confirmed, please login into your account",
                    RoutesEnum::Static_UnAuth_Login.to_string().as_str(),
                ))
            }
        }
    }
}
