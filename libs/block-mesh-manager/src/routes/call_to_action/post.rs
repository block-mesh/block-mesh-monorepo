use crate::database::call_to_action::get_or_create_call_to_action::get_or_create_call_to_action;
use crate::domain::call_to_action::CallToActionName;
use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;
use axum::response::Redirect;
use axum::{Extension, Form};
use axum_login::AuthSession;
use block_mesh_common::interfaces::server_api::CallToActionForm;
use block_mesh_common::routes_enum::RoutesEnum;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use sqlx::PgPool;

#[tracing::instrument(name = "call_to_action", skip_all)]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthSession<Backend>>,
    Form(form): Form<CallToActionForm>,
) -> Result<Redirect, Error> {
    let user = auth.user.ok_or(Error::UserNotFound)?;
    let mut transaction = create_txn(&pool).await?;
    get_or_create_call_to_action(
        &mut transaction,
        user.id,
        CallToActionName::from(form.name),
        form.status,
    )
    .await
    .map_err(Error::from)?;
    commit_txn(transaction).await?;
    Ok(Redirect::to(&format!(
        "/ui{}",
        RoutesEnum::Static_Auth_Dashboard.to_string().as_str()
    )))
}
