use std::str::FromStr;

use axum::{Extension, Json};
use axum_login::AuthSession;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use sqlx::PgPool;

use crate::database::perks::add_perk_to_user::add_perk_to_user;
use crate::database::user::update_user_wallet::update_user_wallet;
use crate::domain::perk::PerkName;
use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;
use block_mesh_common::interfaces::server_api::{ConnectWalletRequest, ConnectWalletResponse};

#[tracing::instrument(name = "connect_wallet", skip(pool, auth))]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthSession<Backend>>,
    Json(body): Json<ConnectWalletRequest>,
) -> Result<Json<ConnectWalletResponse>, Error> {
    let mut transaction = pool.begin().await?;
    let user = auth.user.ok_or(Error::UserNotFound)?;
    let signature =
        Signature::try_from(body.signature.as_slice()).map_err(|_| Error::InternalServer)?;
    let pubkey = Pubkey::from_str(body.pubkey.as_str()).unwrap_or_default();
    let message = body.message.as_bytes();
    if signature.verify(pubkey.as_ref(), message) {
        add_perk_to_user(
            &mut transaction,
            user.id,
            PerkName::Wallet,
            1.1,
            0.0,
            serde_json::from_str("{}").unwrap(),
        )
        .await?;
        update_user_wallet(&mut transaction, user.id, body.pubkey).await?
    } else {
        tracing::error!("Signature verification failed.");
        return Err(Error::SignatureMismatch);
    }
    transaction.commit().await?;
    Ok(Json(ConnectWalletResponse { status: 200 }))
}
