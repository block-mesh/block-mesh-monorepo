use std::net::SocketAddr;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");
    println!("Server listening on {}", addr);
    loop {
        let (mut stream, addr) = listener
            .accept()
            .await
            .expect("Failed to accept connection");
        // let io = TokioIo::new(stream);
        println!("Incoming connection from {}", addr);

        // tokio::spawn(async move {
        //     handle_connection(stream).await;
        // });

        tokio::spawn(async move {
            let (reader, mut writer) = stream.split();
            let mut reader = BufReader::new(reader);

            // Echo loop
            loop {
                let mut buffer = String::new();

                match reader.read_line(&mut buffer).await {
                    Ok(0) => {
                        // End of stream
                        println!("Client disconnected");
                        break;
                    }
                    Ok(_) => {
                        // Echo back to client
                        if let Err(err) = writer.write_all(buffer.as_bytes()).await {
                            println!("Error writing to socket: {}", err);
                            break;
                        }
                    }
                    Err(err) => {
                        println!("Error reading from socket: {}", err);
                        break;
                    }
                }
            }
        });
    }
}

// async fn handle_connection(stream: TcpStream) {
//     // Get the peer address
//     let peer_addr = stream.peer_addr().expect("Failed to get peer address");
//     println!("Peer address: {}", peer_addr);
// }
