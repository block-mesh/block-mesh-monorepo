use axum::{
    extract::ws::{ Message, WebSocket, WebSocketUpgrade },
    response::IntoResponse,
    routing::get,
    Router,
};
use axum_extra::TypedHeader;

use std::borrow::Cow;
use std::ops::ControlFlow;
use std::net::SocketAddr;
use tower_http::trace::{ DefaultMakeSpan, TraceLayer };

//allows to extract the IP of connecting user
use axum::extract::connect_info::ConnectInfo;
use axum::extract::ws::CloseFrame;

//allows to split the websocket stream into separate TX and RX branches
use futures::{ sink::SinkExt, stream::StreamExt };

pub async fn serve_ws() -> std::io::Result<()> {
    let app = Router::new()
        .route("/ws", get(ws_handler))
        // logging so we can see whats going on
        .layer(
            TraceLayer::new_for_http().make_span_with(
                DefaultMakeSpan::default().include_headers(true)
            )
        );
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8001").await.unwrap();
    tracing::info!("Websocket Server Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await
}
async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    tracing::info!("`{user_agent}` at {addr} connected.");
    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.
    ws.on_upgrade(move |socket| handle_socket(socket, addr))
}

/// Actual websocket statemachine (one will be spawned per connection)
async fn handle_socket(socket: WebSocket, who: SocketAddr) {
    let (mut sender, mut receiver) = socket.split();
    let mut send_task = tokio::spawn(async move {
        let n_msg = 50;
        for i in 0..n_msg {
            // In case of any websocket error, we exit.
            if let Err(err) = sender.send(Message::Text(format!("Server message {i} ..."))).await{
                tracing::info!("Error occured while sending message {err}");
            }

            tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
        }
        n_msg
    });

    // This second task will receive messages from client and print them on server console
    let mut recv_task = tokio::spawn(async move {
        let mut cnt = 0;
        while let Some(Ok(msg)) = receiver.next().await {
            cnt += 1;
            // print message and break if instructed to do so
            if process_message(msg, who).is_break() {
                break;
            }
        }
        cnt
    });
}

/// helper to print contents of messages to stdout. Has special treatment for Close.
fn process_message(msg: Message, who: SocketAddr) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(t) => {
            tracing::info!(">>> {who} sent str: {t:?}");
        }
        Message::Close(c) => {
            if let Some(cf) = c {
                tracing::info!(
                    ">>> {} sent close with code {} and reason `{}`",
                    who,
                    cf.code,
                    cf.reason
                );
            } else {
                tracing::info!(">>> {who} somehow sent close message without CloseFrame");
            }
            return ControlFlow::Break(());
        }
        _ => {
            tracing::info!("Message type not supported yet");
        }
    }
    ControlFlow::Continue(())
}
