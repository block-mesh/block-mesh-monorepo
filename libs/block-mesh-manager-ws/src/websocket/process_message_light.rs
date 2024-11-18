use axum::extract::ws::Message;
use std::ops::ControlFlow;

pub fn process_message_light(msg: Message, ip: &str) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(t) => {
            tracing::trace!(">>> {ip} sent str: {t:?}");
        }
        Message::Binary(d) => {
            tracing::trace!(">>> {} sent {} bytes: {:?}", ip, d.len(), d);
        }
        Message::Close(c) => {
            if let Some(cf) = c {
                tracing::trace!(
                    ">>> {} sent close with code {} and reason `{}`",
                    ip,
                    cf.code,
                    cf.reason
                );
            } else {
                tracing::trace!(">>> {ip} somehow sent close message without CloseFrame");
            }
            return ControlFlow::Break(());
        }

        Message::Pong(v) => {
            tracing::trace!(">>> {ip} sent pong with {v:?}");
        }
        // You should never need to manually handle Message::Ping, as axum's websocket library
        // will do so for you automagically by replying with Pong and copying the v according to
        // spec. But if you need the contents of the pings you can see them here.
        Message::Ping(v) => {
            tracing::trace!(">>> {ip} sent ping with {v:?}");
        }
    }
    ControlFlow::Continue(())
}
