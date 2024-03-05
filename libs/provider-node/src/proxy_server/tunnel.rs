use crate::token_management::channels::{ChannelMessage, TX};
use hyper::upgrade::Upgraded;
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;

pub async fn tunnel(upgraded: Upgraded, addr: String) -> std::io::Result<()> {
    let mut server = TcpStream::connect(addr).await?;
    let mut upgraded = TokioIo::new(upgraded);

    let (from_client, from_server) =
        tokio::io::copy_bidirectional(&mut upgraded, &mut server).await?;

    tracing::debug!(
        "client wrote {} bytes and received {} bytes",
        from_client,
        from_server
    );
    println!(
        "client wrote {} bytes and received {} bytes",
        from_client, from_server
    );

    let tx = TX.get().unwrap();
    tx.send(ChannelMessage {
        upload: from_client,
        download: from_server,
        token: Default::default(),
    })
    .unwrap();

    Ok(())
}
