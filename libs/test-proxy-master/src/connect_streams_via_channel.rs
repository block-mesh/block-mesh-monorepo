// #![deny(warnings)]
use crate::Context;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::broadcast::{Receiver, Sender};

pub async fn connect_streams_via_channel(
    listener: TcpListener,
    sender: Sender<Vec<u8>>,
    receiver: Receiver<Vec<u8>>,
    context: Context,
) -> anyhow::Result<()> {
    while let Ok((stream, addr)) = listener.accept().await {
        let sender = sender.clone();
        let mut receiver = receiver.resubscribe();
        println!("{}: Incoming connection from {}", context, addr);
        let (mut read, mut write) = stream.into_split();
        tokio::spawn(async move {
            loop {
                println!("{}: reading data", context);
                let mut buf = vec![];
                let n = read.read_buf(&mut buf).await.expect("Failed to read data");
                println!("{}: read {} bytes", context, n);
                if n == 0 {
                    println!("{}: Connection closed by client", context);
                    break;
                }
                println!("{}: sending msg", context);
                let _ = sender.send(buf);
            }
        });
        tokio::spawn(async move {
            while let Ok(msg) = receiver.recv().await {
                println!("{}: recv msg {:?}", context, msg.len());
                write.write_all(&msg).await.expect("Failed to write data");
            }
        });
    }
    println!("{}: receiver closed", context);
    Ok(())
}
