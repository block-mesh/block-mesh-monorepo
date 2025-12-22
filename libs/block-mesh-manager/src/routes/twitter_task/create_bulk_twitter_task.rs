use crate::errors::error::Error;
use crate::routes::twitter_task::get_twitter_profile_details::get_twitter_profile;
use crate::startup::application::AppState;
use anyhow::anyhow;
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use block_mesh_common::interfaces::server_api::CreateBulkTwitterTask;
use block_mesh_manager_database_domain::domain::twitter_task::TwitterTask;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use http::StatusCode;
use std::env;
use std::sync::Arc;
use time::OffsetDateTime;

#[tracing::instrument(name = "create_bulk_twitter_task", skip_all)]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Query(query): Query<CreateBulkTwitterTask>,
) -> Result<impl IntoResponse, Error> {
    if query.code.is_empty() || query.code != env::var("ADMIN_PARAM").unwrap_or_default() {
        return Err(Error::Anyhow(anyhow!("Bad admin param")));
    }
    let profile = get_twitter_profile(&query.username).await?;
    let since = profile.created_at;
    let until = OffsetDateTime::now_utc().date();
    let mut transaction = create_txn(&state.pool).await?;
    TwitterTask::create_twitter_task(
        &mut transaction,
        &query.username.to_lowercase(),
        &since,
        &until,
    )
    .await?;
    commit_txn(transaction).await?;
    Ok((StatusCode::OK, "OK").into_response())
}
