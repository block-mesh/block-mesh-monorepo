use crate::database::perks::add_perk_to_user::add_perk_to_user;
use crate::database::user::update_proof_of_human::update_proof_of_human;
use crate::domain::perk::PerkName;
use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;
use crate::notification::notification_redirect::NotificationRedirect;
use crate::startup::application::AppState;
use crate::utils::cftoken::check_cf_token;
use crate::utils::hcaptcha::hcaptcha;
use crate::utils::recaptcha_v2::recaptcha_v2;
use askama_axum::IntoResponse;
use axum::extract::State;
use axum::{Extension, Form};
use axum_login::AuthSession;
use block_mesh_common::interfaces::server_api::ProofOfHumanForm;
use block_mesh_common::routes_enum::RoutesEnum;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use std::sync::Arc;

#[tracing::instrument(name = "proof_of_humanity_post", skip_all)]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthSession<Backend>>,
    Form(form): Form<ProofOfHumanForm>,
) -> Result<impl IntoResponse, Error> {
    let user = auth.user.ok_or(Error::UserNotFound)?;
    if let Err(e) = check_cf_token(form.cftoken, &state.cf_secret_key).await {
        tracing::info!("CF Failed {}", e);
        return Ok(Error::redirect(
            500,
            "CF Captcha Error",
            "Failed to prove you are human",
            RoutesEnum::Static_Auth_Proof_Of_Humanity
                .to_string()
                .as_str(),
        ));
    }
    if let Err(e) = recaptcha_v2(form.recaptcha_v2, &state.recaptcha_secret_key_v2).await {
        tracing::info!("ReCaptcha V2 Failed {}", e);
        return Ok(Error::redirect(
            500,
            "ReCaptcha V2 Error",
            "Failed to prove you are human",
            RoutesEnum::Static_Auth_Proof_Of_Humanity
                .to_string()
                .as_str(),
        ));
    }
    if let Err(e) = hcaptcha(form.hcaptcha, &state.hcaptcha_secret_key).await {
        tracing::info!("HCaptcha  Failed {}", e);
        return Ok(Error::redirect(
            500,
            "HCaptcha Error",
            "Failed to prove you are human",
            RoutesEnum::Static_Auth_Proof_Of_Humanity
                .to_string()
                .as_str(),
        ));
    }
    let mut transaction = create_txn(&state.pool).await?;
    update_proof_of_human(&mut transaction, user.id, true).await?;
    add_perk_to_user(
        &mut transaction,
        user.id,
        PerkName::ProofOfHumanity,
        1.0,
        1_000.0,
        serde_json::from_str("{}").unwrap(),
    )
    .await?;
    commit_txn(transaction).await?;
    Ok(NotificationRedirect::redirect(
        "Success",
        "Human Verification done",
        "/ui/perks",
    ))
}
