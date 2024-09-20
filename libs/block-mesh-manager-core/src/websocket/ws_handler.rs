use crate::errors::Error;
use anyhow::Context;
use axum::response::IntoResponse;
use axum::{
    extract::{Query, State, WebSocketUpgrade},
    http::HeaderMap,
};

use crate::websocket::handle_socket::handle_socket;
use crate::AppState;
use block_mesh_common::env::environment::Environment;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

pub async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}
