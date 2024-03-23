use clap::Parser;
use std::time::Duration;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long, default_value = "127.0.0.1")]
    pub ip: String,
    #[arg(long, default_value = "3000")]
    pub port: u16,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Args::parse();
    let stream = TcpStream::connect(format!("{}:{}", args.ip, args.port)).await?;
    let (mut reader, mut writer) = stream.into_split();

    // Spawn a Tokio task to periodically write to the stream.
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(5)); // Write every 5 seconds

        loop {
            interval.tick().await;
            match writer.write_all(b"Ping from client!\n").await {
                Ok(_) => println!("Sent ping to server."),
                Err(e) => eprintln!("Error writing to stream: {}", e),
            }
        }
    });

    loop {
        let mut buffer = [0; 1024];
        match reader.read(&mut buffer).await {
            // If successful, print received data.
            Ok(n) if n > 0 => {
                let string = String::from_utf8_lossy(&buffer);
                let string = string.trim_end_matches(|c| c == '\0' || c == '\n');
                println!("Received: {:?}", string);
            }
            // If connection closed by the server, break out of the loop.
            Ok(0) => {
                println!("Connection closed by server.");
                break;
            }
            // Handle read error.
            Err(e) => {
                eprintln!("Error reading from stream: {}", e);
                break;
            }
            _ => {
                println!("_ case Received: {:?}", &buffer[..0]);
            }
        }
    }

    Ok(())
}
