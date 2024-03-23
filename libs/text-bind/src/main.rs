use std::net::SocketAddr;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::time;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");
    println!("Server listening on {}", addr);
    loop {
        let (stream, addr) = listener
            .accept()
            .await
            .expect("Failed to accept connection");
        // let io = TokioIo::new(stream);
        println!("Incoming connection from {}", addr);

        // tokio::spawn(async move {
        //     handle_connection(stream).await;
        // });

        tokio::spawn(async move {
            let (reader, mut writer) = stream.into_split();
            let mut reader = BufReader::new(reader);

            tokio::spawn(async move {
                let mut interval = time::interval(Duration::from_secs(5)); // Write every 5 seconds

                loop {
                    interval.tick().await;
                    //  let request = "GET / HTTP/1.1\r\n\
                    // Host: example.com\r\n\
                    // Connection: close\r\n\
                    // \r\n";
                    let request = "GET http://example.com/ HTTP/1.1\r\n\
                   Host: example.com\r\n\
                   Connection: close\r\n\
                   \r\n";
                    println!("Sending ping to server.");

                    match writer.write_all(request.as_bytes()).await {
                        Ok(_) => println!("Sent ping to server."),
                        Err(e) => eprintln!("Error writing to stream: {}", e),
                    }
                }
            });

            // Echo loop
            loop {
                let mut buffer = String::new();

                match reader.read_line(&mut buffer).await {
                    Ok(0) => {
                        // End of stream
                        println!("Client disconnected");
                        break;
                    }
                    Ok(n) if n > 0 => {
                        let string = buffer.trim_end_matches(|c| c == '\0');
                        println!("1 Received: {:?}", string);
                    }
                    Ok(_) => {
                        println!("2 Received: {:?}", &buffer[..0]);
                        // Echo back to client
                        // if let Err(err) = writer.write_all(buffer.as_bytes()).await {
                        //     println!("Error writing to socket: {}", err);
                        //     break;
                        // }
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
