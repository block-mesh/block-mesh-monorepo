//! Based on tokio-tungstenite example websocket client, but with multiple
//! concurrent websocket clients in one package
//!
//! This will connect to a server specified in the SERVER with N_CLIENTS
//! concurrent connections, and then flood some test messages over websocket.
//! This will also print whatever it gets into stdout.
//!
//! Note that this is not currently optimized for performance, especially around
//! stdout mutex management. Rather it's intended to show an example of working with axum's
//! websocket server and how the client-side and server-side code can be quite similar.
//!

use futures_util::stream::FuturesUnordered;
use futures_util::{SinkExt, StreamExt};
use std::ops::ControlFlow;
use std::time::Instant;
// we will use tungstenite for websocket client impl (same library as what axum is using)
use crate::SIGNAL_TX;
use tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode;
use tokio_tungstenite::tungstenite::protocol::CloseFrame;
use tokio_tungstenite::tungstenite::Utf8Bytes;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use uuid::Uuid;

const SERVER: &str = "ws://localhost:8002/ws";

pub async fn run_client(num_clients: usize) {
    let start_time = Instant::now();
    //spawn several clients that will concurrently talk to the server
    let mut clients = (0..num_clients)
        .map(|cli| tokio::spawn(spawn_client(cli)))
        .collect::<FuturesUnordered<_>>();

    //wait for all our clients to exit
    while clients.next().await.is_some() {}

    let end_time = Instant::now();

    //total time should be the same no matter how many clients we spawn
    println!(
        "Total time taken {:#?} with {num_clients} concurrent clients, should be about 6.45 seconds.",
        end_time - start_time
    );
}

//creates a client. quietly exits on failure.
async fn spawn_client(who: usize) {
    let url = format!(
        "{}?email={}@gmail.com&api_token={}",
        SERVER,
        who,
        Uuid::new_v4()
    );
    let ws_stream = match connect_async(url).await {
        Ok((stream, _response)) => {
            // println!("Handshake for client {who} has been completed");
            // This will be the HTTP response, same as with server this is the last moment we
            // can still access HTTP stuff.
            // println!("Server response was {response:?}");
            stream
        }
        Err(e) => {
            println!("WebSocket handshake for client {who} failed with {e}!");
            return;
        }
    };

    let (mut sender, mut receiver) = ws_stream.split();

    //we can ping the server for start
    sender
        .send(Message::Ping(axum::body::Bytes::from_static(
            b"Hello, Server!",
        )))
        .await
        .expect("Can not send!");

    //spawn an async sender to push some more messages into the server
    let mut send_task = tokio::spawn(async move {
        if let Some(tx) = SIGNAL_TX.get() {
            let mut rx = tx.subscribe();
            while let Ok(_) = rx.recv().await {
                eprintln!("recv_async Closing {who}");
                let _ = sender
                    .send(Message::Close(Some(CloseFrame {
                        code: CloseCode::Normal,
                        reason: Utf8Bytes::from_static("Goodbye"),
                    })))
                    .await;
            }
        }
    });

    //receiver just prints whatever it gets
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            // print message and break if instructed to do so
            if process_message(msg, who).is_break() {
                break;
            }
        }
    });

    //wait for either task to finish and kill the other task
    tokio::select! {
        _ = (&mut send_task) => {
            println!("send_task for {} ended", who);
            recv_task.abort();
        },
        _ = (&mut recv_task) => {
            println!("rect_task for {} ended", who);
            send_task.abort();
        }
    }
}

/// Function to handle messages we get (with a slight twist that Frame variant is visible
/// since we are working with the underlying tungstenite library directly without axum here).
fn process_message(msg: Message, who: usize) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(_t) => {
            // println!(">>> {who} got str: {t:?}");
        }
        Message::Binary(d) => {
            println!(">>> {} got {} bytes: {:?}", who, d.len(), d);
        }
        Message::Close(c) => {
            if let Some(cf) = c {
                println!(
                    ">>> {} got close with code {} and reason `{}`",
                    who, cf.code, cf.reason
                );
            } else {
                println!(">>> {who} somehow got close message without CloseFrame");
            }
            return ControlFlow::Break(());
        }

        Message::Pong(_v) => {
            // println!(">>> {who} got pong with {v:?}");
        }
        // Just as with axum server, the underlying tungstenite websocket library
        // will handle Ping for you automagically by replying with Pong and copying the
        // v according to spec. But if you need the contents of the pings you can see them here.
        Message::Ping(_v) => {
            // println!(">>> {who} got ping with {v:?}");
        }

        Message::Frame(_) => {
            unreachable!("This is never supposed to happen")
        }
    }
    ControlFlow::Continue(())
}
