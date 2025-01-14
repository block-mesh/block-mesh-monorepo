use axum::{response::Response, routing::get, Router};
use axum_tws::{Message, WebSocket, WebSocketUpgrade};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("pid = {}", std::process::id());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    axum::serve(listener, Router::new().route("/ws", get(handle_upgrade))).await?;

    Ok(())
}

async fn handle_upgrade(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade({
        move |socket| async {
            if let Err(e) = handle_ws(socket).await {
                println!("websocket error: {:?}", e);
            }
        }
    })
}

async fn handle_ws(mut socket: WebSocket) -> Result<(), Box<dyn std::error::Error>> {
    while let Some(Ok(msg)) = socket.recv().await {
        if msg.is_text() {
            socket.send(msg).await?;
            println!("Received text message");
        } else if msg.is_ping() {
            println!("Receieved ping");
            let payload = msg.into_payload();
            if let Err(e) = socket.send(Message::pong(payload)).await {
                eprintln!("{}", e);
                break;
            }
        } else if msg.is_close() {
            println!("Received close frame");
            break;
        }
    }

    Ok(())
}
