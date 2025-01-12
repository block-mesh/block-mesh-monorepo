use axum::{
	extract::ws::{Message, WebSocket, WebSocketUpgrade},
	response::IntoResponse,
	routing::get,
	Router,
};
use futures::{SinkExt, StreamExt};
use std::net::SocketAddr;
use tokio::net::TcpListener;

use log::info;

/*
#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;
*/

#[tokio::main]
async fn main() {
	env_logger::init();
	let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
	let listener = TcpListener::bind(addr).await.unwrap();
	let app = Router::new().route("/ws", get(ws_handler));
	axum::serve(listener, app).await.unwrap();
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
	ws.on_upgrade(handle_socket)
}

async fn handle_socket(socket: WebSocket) {
	let (mut tx, mut rx) = socket.split();
	while let Some(Ok(msg)) = rx.next().await {
		match msg {
			Message::Text(text) => {
				println!("Received text message: {}", text);
			}
			Message::Ping(payload) => {
				info!("Receieved connection");
				if let Err(e) = tx.send(Message::Pong(payload)).await {
					eprintln!("{}", e);
					break;
				}
			}
			Message::Close(close_frame) => {
				println!("Received close frame: {:?}", close_frame);
				break;
			}
			_ => {
				// Ignore other message types (Binary, Pong, etc.).
			}
		}
	}

	info!("Client disconnected");
}
