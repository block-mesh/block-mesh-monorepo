use crate::errors::error::Error;
use crate::startup::application::AppState;
use crate::utils::cache_envar::get_envar;
use anyhow::anyhow;
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::Json;
use block_mesh_common::interfaces::server_api::DebugEndpoint;
use block_mesh_manager_database_domain::domain::bulk_get_or_create_aggregate_by_user_and_name::{
    bulk_get_or_create_aggregate_by_user_and_name,
    bulk_get_or_create_aggregate_by_user_and_name_old,
};
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use std::sync::Arc;

#[tracing::instrument(name = "debug_endpoint", skip_all)]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Query(query): Query<DebugEndpoint>,
) -> Result<impl IntoResponse, Error> {
    let app_env = get_envar("APP_ENVIRONMENT").await;
    let admin_param = get_envar("ADMIN_PARAM").await;
    if app_env != "local" {
        return Err(Error::InternalServer);
    }
    if query.code.is_empty() || query.code != admin_param {
        return Err(Error::InternalServer);
    }

    let mut transaction = create_txn(&state.pool).await?;
    let results = if query.method == "bulk_get_or_create_aggregate_by_user_and_name" {
        bulk_get_or_create_aggregate_by_user_and_name(&mut transaction, &query.user_id).await
    } else if query.method == "bulk_get_or_create_aggregate_by_user_and_name_old" {
        bulk_get_or_create_aggregate_by_user_and_name_old(&mut transaction, &query.user_id).await
    } else {
        return Err(Error::InternalServer);
    }?;
    let value = serde_json::to_value(results).map_err(|e| anyhow!(e))?;
    let json = Json(value);

    commit_txn(transaction).await?;
    Ok(json.into_response())
}
