use crate::database::nonce::get_nonce_by_nonce::get_nonce_by_nonce;
use crate::database::nonce::update_nonce::update_nonce;
use crate::database::spam_email::get_spam_emails::get_spam_emails_cache;
use crate::database::user::update_email::update_email;
use crate::database::user::update_verified_email::update_verified_email;
use crate::domain::spam_email::SpamEmail;
use crate::errors::error::Error;
use axum::extract::Query;
use axum::Extension;
use axum::Json;
use block_mesh_common::interfaces::server_api::ConfirmEmailRequest;
use block_mesh_manager_database_domain::domain::get_user_opt_by_id::get_user_opt_by_id;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize)]
pub struct EmailConfirmResponse {
    pub success: bool,
    pub message: String,
}

#[tracing::instrument(name = "email_confirm_api", skip_all)]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Query(query): Query<ConfirmEmailRequest>,
) -> Result<Json<EmailConfirmResponse>, Error> {
    let mut transaction = create_txn(&pool).await?;

    let nonce = get_nonce_by_nonce(&mut transaction, &query.token)
        .await
        .map_err(|_| Error::BadRequest("Invalid token".to_string()))?;

    let email = query.email.clone().to_ascii_lowercase();
    let spam_emails = get_spam_emails_cache().await;
    let email_domain = match email.split('@').next_back() {
        Some(d) => d.to_string(),
        None => {
            commit_txn(transaction).await?;
            return Err(Error::BadRequest("Invalid email domain".to_string()));
        }
    };

    if SpamEmail::check_domains(&email_domain, spam_emails).is_err() {
        commit_txn(transaction).await?;
        return Err(Error::BadRequest("Invalid email domain".to_string()));
    }

    match nonce {
        None => {
            commit_txn(transaction).await?;
            Err(Error::BadRequest("Token not found".to_string()))
        }
        Some(nonce) => {
            if *nonce.nonce.expose_secret() != query.token {
                commit_txn(transaction).await?;
                return Err(Error::BadRequest("Token mismatch".to_string()));
            }

            let user = get_user_opt_by_id(&mut transaction, &nonce.user_id)
                .await
                .map_err(Error::from)?;

            if user.is_none() {
                commit_txn(transaction).await?;
                return Err(Error::UserNotFound);
            }

            let user = user.unwrap();

            update_verified_email(&mut transaction, &user.id, true)
                .await
                .map_err(Error::from)?;

            if user.email != email {
                update_email(&mut transaction, &user.id, &email)
                    .await
                    .map_err(Error::from)?;
            }

            update_nonce(&mut transaction, user.id)
                .await
                .map_err(|_| Error::BadRequest("Failed to update nonce".to_string()))?;

            commit_txn(transaction).await?;

            Ok(Json(EmailConfirmResponse {
                success: true,
                message: "Email confirmed successfully. Please login.".to_string(),
            }))
        }
    }
}
