use crate::state::AppState;
use crate::websocket::handle_socket::handle_socket;
use axum::extract::{State, WebSocketUpgrade};
use axum::response::IntoResponse;

pub async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}
