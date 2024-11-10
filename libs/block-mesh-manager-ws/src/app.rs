use crate::errors::Error;
use crate::state::AppState;
use crate::websocket::ws_handler::ws_handler;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use block_mesh_manager_database_domain::domain::task_limit::TaskLimit;
use database_utils::utils::health_check::health_check;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

#[tracing::instrument(name = "health", skip_all)]
pub async fn health(State(state): State<Arc<AppState>>) -> Result<impl IntoResponse, Error> {
    let pool = state.pool.clone();
    let mut transaction = create_txn(&pool).await?;
    health_check(&mut *transaction).await?;
    commit_txn(transaction).await?;
    Ok((StatusCode::OK, "OK"))
}

#[tracing::instrument(name = "health_follower", skip_all)]
pub async fn health_follower(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    let pool = state.follower_pool.clone();
    let mut transaction = create_txn(&pool).await?;
    health_check(&mut *transaction).await?;
    commit_txn(transaction).await?;
    Ok((StatusCode::OK, "OK"))
}

#[derive(Deserialize)]
pub struct AdminParam {
    code: String,
}

pub async fn summary(
    State(state): State<Arc<AppState>>,
    Query(admin_param): Query<AdminParam>,
) -> Result<Json<Value>, Error> {
    if admin_param.code != env::var("ADMIN_PARAM").unwrap_or_default() {
        Err(Error::InternalServer("Bad admin param".to_string()))
    } else {
        let sockets: Vec<String> = state
            .websocket_manager
            .broadcaster
            .sockets
            .iter()
            .map(|i| i.key().0.clone().to_string())
            .collect();
        Ok(Json(Value::from(sockets)))
    }
}

pub async fn detailed_summary(
    State(state): State<Arc<AppState>>,
    Query(admin_param): Query<AdminParam>,
) -> Result<Json<Vec<Value>>, Error> {
    if admin_param.code != env::var("ADMIN_PARAM").unwrap_or_default() {
        Err(Error::InternalServer("Bad admin param".to_string()))
    } else {
        let task_limit = env::var("TASK_LIMIT")
            .unwrap_or("10".to_string())
            .parse()
            .unwrap_or(10);
        let mut redis = state.redis.clone();
        let mut limits: Vec<Value> = Vec::with_capacity(10_000);
        for socket in state.websocket_manager.broadcaster.sockets.iter() {
            let user_id = socket.key().0;
            if let Ok(task_limit) =
                TaskLimit::get_task_limit(&user_id, &mut redis, task_limit).await
            {
                let v: Value = task_limit.into();
                limits.push(v);
            }
        }
        Ok(Json(limits))
    }
}

#[tracing::instrument(name = "version", skip_all)]
pub async fn version() -> impl IntoResponse {
    (StatusCode::OK, env!("CARGO_PKG_VERSION"))
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StatsResponse {
    queue: usize,
}

#[tracing::instrument(name = "stats", skip_all)]
pub async fn stats(State(state): State<Arc<AppState>>) -> Json<StatsResponse> {
    let websocket_manager = &state.websocket_manager;
    let queue = websocket_manager.broadcaster.queue.lock().await;
    Json(StatsResponse { queue: queue.len() })
}

pub async fn app(listener: TcpListener, state: Arc<AppState>) {
    let router = Router::new()
        .route("/", get(health))
        .route("/health", get(health))
        .route("/health_follower", get(health_follower))
        .route("/version", get(version))
        .route("/stats", get(stats))
        .route("/summary", get(summary))
        .route("/detailed_summary", get(detailed_summary))
        .route("/ws", get(ws_handler))
        .with_state(state);

    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
