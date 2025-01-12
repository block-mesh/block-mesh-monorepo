use clap::Parser;
use futures::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, WebSocketStream};
use serde::{Deserialize};

const URL: &str = "ws://127.0.0.1:3000/ws";

#[derive(Parser, Clone, Deserialize)]
pub struct Options {
	#[clap(long)]
	pub num_clients: usize,
}

#[tokio::main]
async fn main() {
	let args = Options::parse();
	let mut handles = Vec::new();
	for _ in 0..args.num_clients {
		let handle = tokio::spawn(async {
			match connect_async(URL).await {
				Ok((stream, _)) =>  handle_stream(stream).await,
				Err(e) => eprintln!("Connect failed: '{}'", e),
			}
		});
		handles.push(handle);
	}
	for handle in handles {
		handle.await.expect("panic in task");
	}
}

async fn handle_stream(
	ws_stream: WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
) {
	let (mut sender, mut receiver) = ws_stream.split();
	sender
		.send(Message::Ping(axum::body::Bytes::from_static(
			b"Hello, Server!",
		)))
		.await
		.expect("Cannot send!");
	loop {
		receiver.next().await;
	}
}
