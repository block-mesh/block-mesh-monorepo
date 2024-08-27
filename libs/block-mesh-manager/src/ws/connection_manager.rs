use axum::extract::ws::{Message, WebSocket};
use dashmap::DashMap;
use futures::stream::SplitSink;
use futures::{Sink, SinkExt};
use std::fmt::{Display, Formatter};
use std::sync::Arc;
use tracing::{error, trace, warn};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ConnectionManager(Arc<DashMap<Uuid, Session>>);

impl ConnectionManager {
    pub fn new() -> Self {
        Self(Arc::new(DashMap::new()))
    }
    pub fn register(&self, user_id: Uuid, transmitter: SplitSink<WebSocket, Message>) {
        if let Some(old_value) = self.0.insert(user_id, Session::new(transmitter)) {
            error!("Registered already existing session for {user_id}");
        }
    }
    pub async fn close_session(&self, session_id: &Uuid) {
        if let Some((_, mut session)) = self.0.remove(session_id) {
            trace!("Sending closing message to session {session_id}");
            session.close().await.unwrap();
            trace!("Session {session_id} was successfully closed (by order)")
        } else {
            warn!("Session {session_id} ordered to be closed is missing from connection manager (perhaps it could be closed already)")
        }
    }
}

impl Display for ConnectionManager {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let dump: Vec<String> = self.0.iter().map(|entry| entry.key().to_string()).collect();
        write!(f, "Online sessions ({}): {}", dump.len(), dump.join(", "))
    }
}

#[derive(Debug)]
pub struct Session {
    transmitter: SplitSink<WebSocket, Message>,
}

impl Session {
    fn new(transmitter: SplitSink<WebSocket, Message>) -> Self {
        Self { transmitter }
    }
    async fn close(&mut self) -> Result<(), axum::Error> {
        self.transmitter
            .send(Message::Text(format!("close")))
            .await?;
        let close = self.transmitter.close().await.unwrap(); // waits for the sink to close
        Ok(())
    }
}
