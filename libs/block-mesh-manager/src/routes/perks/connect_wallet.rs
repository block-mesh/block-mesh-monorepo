use crate::database::perks::add_perk_to_user::add_perk_to_user;
use crate::database::user::get_user_by_email::get_user_opt_by_email;
use crate::database::user::update_user_wallet::update_user_wallet;
use crate::domain::perk::PerkName;
use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;
use crate::startup::application::AppState;
use axum::extract::State;
use axum::{Extension, Json};
use axum_login::AuthSession;
use block_mesh_common::interfaces::server_api::{ConnectWalletRequest, ConnectWalletResponse};
use block_mesh_manager_database_domain::domain::aggregate::AggregateName;
use block_mesh_manager_database_domain::domain::get_or_create_aggregate_by_user_and_name::get_or_create_aggregate_by_user_and_name_str;
use block_mesh_manager_database_domain::domain::update_aggregate::update_aggregate;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use sqlx::PgPool;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

#[tracing::instrument(name = "connect_wallet", skip_all)]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthSession<Backend>>,
    Json(body): Json<ConnectWalletRequest>,
) -> Result<Json<ConnectWalletResponse>, Error> {
    let mut transaction = pool.begin().await?;
    let user = auth.user.ok_or(Error::UserNotFound)?;
    let db_user = get_user_opt_by_email(&mut transaction, &user.email)
        .await?
        .ok_or(Error::UserNotFound)?;
    let signature =
        Signature::try_from(body.signature.as_slice()).map_err(|_| Error::InternalServer)?;
    let pubkey = Pubkey::from_str(body.pubkey.as_str()).unwrap_or_default();
    let message = body.message.as_bytes();
    if signature.verify(pubkey.as_ref(), message) {
        match db_user.wallet_address {
            None => {
                add_perk_to_user(
                    &mut transaction,
                    user.id,
                    PerkName::Wallet,
                    1.1,
                    0.0,
                    serde_json::from_str("{}").unwrap(),
                )
                .await?;
                update_user_wallet(&mut transaction, user.id, &body.pubkey).await?;
                state
                    .wallet_addresses
                    .insert(user.email.clone(), Some(body.pubkey));
            }
            Some(wallet_address) => {
                let name = format!("{}_{}", AggregateName::WalletChange, Uuid::new_v4());
                let agg = get_or_create_aggregate_by_user_and_name_str(
                    &mut transaction,
                    &name,
                    &db_user.id,
                )
                .await?;
                let _ = update_aggregate(&mut transaction, &agg.id, &Value::from(wallet_address))
                    .await?;
                update_user_wallet(&mut transaction, user.id, &body.pubkey).await?;
                state
                    .wallet_addresses
                    .insert(user.email.clone(), Some(body.pubkey));
            }
        }
    } else {
        tracing::error!("Signature verification failed.");
        return Err(Error::SignatureMismatch);
    }
    transaction.commit().await?;
    Ok(Json(ConnectWalletResponse { status: 200 }))
}
