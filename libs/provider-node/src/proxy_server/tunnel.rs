use crate::app_state::AppState;
use crate::token_management::channels::{send_message, ChannelMessage};
use hyper::upgrade::Upgraded;
use hyper_util::rt::TokioIo;
use std::sync::Arc;
use tokio::net::TcpStream;

#[tracing::instrument(name = "tunnel", skip(), ret, err)]
pub async fn tunnel(
    app_state: Arc<AppState>,
    upgraded: Upgraded,
    addr: String,
) -> std::io::Result<()> {
    let mut server = TcpStream::connect(addr).await?;
    let mut upgraded = TokioIo::new(upgraded);
    let (from_client, from_server) =
        tokio::io::copy_bidirectional(&mut upgraded, &mut server).await?;
    tracing::debug!(
        "client wrote {} bytes and received {} bytes",
        from_client,
        from_server
    );
    send_message(
        &app_state.tx,
        ChannelMessage {
            upload: from_client,
            download: from_server,
            token: Default::default(),
        },
    );
    Ok(())
}
