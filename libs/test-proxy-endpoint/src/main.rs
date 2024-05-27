use block_mesh_common::http::{empty, full, host_addr};
use block_mesh_common::tracing::setup_tracing;
use bytes::Bytes;
use clap::Parser;
use http::header;
use http_body_util::{combinators::BoxBody, BodyExt};
use hyper::client::conn::http1::Builder;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::upgrade::Upgraded;
use hyper::{client, http, Method, Request, Response};
use hyper_util::rt::TokioIo;
use std::net::SocketAddr;
use std::str::FromStr;
use tokio::net::TcpStream;
use uuid::Uuid;

#[derive(Parser, Debug)]
pub struct CliArgs {
    #[arg(long, default_value = "127.0.0.1")]
    pub ip: String,
    #[arg(long, default_value = "5000")]
    pub port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_tracing(Uuid::new_v4());
    let args = CliArgs::parse();
    let addr = SocketAddr::from_str(format!("{}:{}", args.ip, args.port).as_str())
        .expect("Failed to parse address");
    while let Ok(stream) = TcpStream::connect(addr).await {
        tracing::info!("Connected to {}", addr);
        // Initial registration
        let (mut send_request, conn) = client::conn::http1::Builder::new()
            .handshake(TokioIo::new(stream))
            .await?;

        tokio::spawn(conn.with_upgrades());

        // TODO: register proxy-endpoint_node in proxy-master

        // let req = Request::builder()
        //     .method(Method::POST)
        //     // whatever
        //     .uri(addr.to_string())
        //     .header(header::UPGRADE, "foobar")
        //     .header("custom-header", "I want connect xxx")
        //     .body(empty())
        //     .unwrap();
        // let _res = send_request.send_request(req).await?;

        let req = Request::builder()
            .method(Method::CONNECT)
            // whatever
            .uri(addr.to_string())
            .header(header::UPGRADE, "")
            .header("custom-header", "I want connect xxx")
            .body(empty())
            .unwrap();

        let res = send_request.send_request(req).await?;

        let stream = hyper::upgrade::on(res).await?;

        // Start Proxy
        if let Err(err) = http1::Builder::new()
            .preserve_header_case(true)
            .title_case_headers(true)
            .serve_connection(stream, service_fn(proxy))
            .with_upgrades()
            .await
        {
            tracing::info!("Failed to serve connection: {:?}", err);
        }
    }
    Ok(())
}

#[tracing::instrument(name = "proxy", ret, err)]
async fn proxy(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    tracing::info!("req: {:?}", req);

    if Method::CONNECT == req.method() {
        // Received an HTTP request like:
        // ```
        // CONNECT www.domain.com:443 HTTP/1.1
        // Host: www.domain.com:443
        // Proxy-Connection: Keep-Alive
        // ```
        //
        // When HTTP method is CONNECT we should return an empty body
        // then we can eventually upgrade the connection and talk a new protocol.
        //
        // Note: only after client received an empty body with STATUS_OK can the
        // connection be upgraded, so we can't return a response inside
        // `on_upgrade` future.
        if let Some(addr) = host_addr(req.uri()) {
            tokio::task::spawn(async move {
                match hyper::upgrade::on(req).await {
                    Ok(upgraded) => {
                        if let Err(e) = tunnel(upgraded, addr).await {
                            tracing::error!("server io error: {}", e);
                        };
                    }
                    Err(e) => tracing::error!("upgrade error: {}", e),
                }
            });

            Ok(Response::new(empty()))
        } else {
            tracing::error!("CONNECT host is not socket addr: {:?}", req.uri());
            let mut resp = Response::new(full("CONNECT must be to a socket address"));
            *resp.status_mut() = http::StatusCode::BAD_REQUEST;

            Ok(resp)
        }
    } else {
        tracing::info!("NOT CONNECT request");
        let host = req.uri().host().expect("uri has no host");
        let port = req.uri().port_u16().unwrap_or(80);

        let stream = TcpStream::connect((host, port)).await.unwrap();
        let io = TokioIo::new(stream);

        let (mut sender, conn) = Builder::new()
            .preserve_header_case(true)
            .title_case_headers(true)
            .handshake(io)
            .await?;
        tokio::task::spawn(async move {
            if let Err(err) = conn.await {
                tracing::info!("Connection failed: {:?}", err);
            }
        });

        let resp = sender.send_request(req).await?;
        Ok(resp.map(|b| b.boxed()))
    }
}

// Create a TCP connection to host:port, build a tunnel between the connection and
// the upgraded connection
#[tracing::instrument(name = "tunnel", ret, err)]
async fn tunnel(upgraded: Upgraded, addr: String) -> std::io::Result<()> {
    let mut server = TcpStream::connect(addr.clone()).await?;
    let mut upgraded = TokioIo::new(upgraded);
    let (from_client, from_server) =
        tokio::io::copy_bidirectional(&mut upgraded, &mut server).await?;
    tracing::info!(from_client, from_server);
    Ok(())
}
