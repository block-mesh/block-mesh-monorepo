use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;
use crate::routes::dashboard::dashboard_data_extractor::dashboard_data_extractor;
use crate::startup::application::AppState;
use axum::extract::State;
use axum::{Extension, Json};
use axum_login::AuthSession;
use block_mesh_common::interfaces::db_messages::{AggregateAddToMessage, DBMessageTypes};
use block_mesh_common::interfaces::server_api::DashboardResponse;
use block_mesh_manager_database_domain::domain::aggregate::AggregateName;
use block_mesh_manager_database_domain::domain::notify_worker::notify_worker;
use sqlx::PgPool;
use std::sync::Arc;
#[allow(unused_imports)]
use tracing::Level;

#[tracing::instrument(name = "dashboard", skip_all)]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<AuthSession<Backend>>,
) -> Result<Json<DashboardResponse>, Error> {
    let user = auth.user.ok_or(Error::UserNotFound)?;
    let _ = notify_worker(
        &pool,
        AggregateAddToMessage {
            msg_type: DBMessageTypes::AggregateAddToMessage,
            user_id: user.id,
            value: serde_json::Value::from(1),
            name: AggregateName::Uptime.to_string(),
        },
    )
    .await;
    let data = dashboard_data_extractor(&pool, user.id, state).await?;
    Ok(Json(data))
}
