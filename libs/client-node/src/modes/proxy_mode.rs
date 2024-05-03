use axum::body::Bytes;
use axum::http::{header, HeaderValue};
use block_mesh_common::cli::ClientNodeOptions;
use block_mesh_common::http::{empty, full, host_addr};
use block_mesh_solana_client::helpers::sign_message;
use block_mesh_solana_client::manager::{FullRouteHeader, SolanaManager};
use http_body_util::combinators::BoxBody;
use http_body_util::BodyExt;
use hyper::client::conn::http1::Builder;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::upgrade::Upgraded;
use hyper::{client, http, Method, Request, Response};
use hyper_util::rt::TokioIo;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use uuid::Uuid;

#[tracing::instrument(name = "proxy_mode", skip(solana_manager), ret, err)]
pub async fn proxy_mode(
    solana_manager: Arc<SolanaManager>,
    proxy_url: Arc<String>,
    client_node_cli_args: &ClientNodeOptions,
) -> anyhow::Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], client_node_cli_args.proxy_port));
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let proxy_url = proxy_url.clone();
        let solana_manager = solana_manager.clone();
        tokio::task::spawn(async move {
            let io = TokioIo::new(stream);
            if let Err(err) = http1::Builder::new()
                .preserve_header_case(true)
                .title_case_headers(true)
                .serve_connection(
                    io,
                    service_fn(move |req| proxy(req, solana_manager.clone(), proxy_url.clone())),
                )
                .with_upgrades()
                .await
            {
                println!("Failed to serve connection: {:?}", err);
            }
        });
    }
    Ok(())
}

#[tracing::instrument(name = "proxy", skip(solana_manager), ret, err)]
async fn proxy(
    mut req: Request<hyper::body::Incoming>,
    solana_manager: Arc<SolanaManager>,
    proxy_url: Arc<String>,
) -> anyhow::Result<Response<BoxBody<Bytes, hyper::Error>>> {
    let nonce = Uuid::new_v4().to_string();
    let signed_message = sign_message(&nonce, &solana_manager.get_keypair())?;
    let solana_manager_header = FullRouteHeader::new(
        nonce,
        signed_message,
        solana_manager.get_pubkey(),
        solana_manager.get_api_token(),
        "client-node".to_string(),
    );
    let json = serde_json::to_string(&solana_manager_header)?;
    let proxy_authorization = HeaderValue::from_str(&json)?;
    req.headers_mut()
        .insert("Proxy-Authorization", proxy_authorization.clone());
    println!("req: {:?}", req);
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
                        if let Err(e) =
                            tunnel(upgraded, addr, proxy_url.clone(), proxy_authorization).await
                        {
                            eprintln!("server io error: {}", e);
                        };
                    }
                    Err(e) => eprintln!("upgrade error: {}", e),
                }
            });

            Ok(Response::new(empty()))
        } else {
            eprintln!("CONNECT host is not socket addr: {:?}", req.uri());
            let mut resp = Response::new(full("CONNECT must be to a socket address"));
            *resp.status_mut() = http::StatusCode::BAD_REQUEST;

            Ok(resp)
        }
    } else {
        let host = req.uri().host().expect("uri has no host");
        let port = req.uri().port_u16().unwrap_or(80);

        let stream = TcpStream::connect((host, port)).await?;
        let io = TokioIo::new(stream);

        let (mut sender, conn) = Builder::new()
            .preserve_header_case(true)
            .title_case_headers(true)
            .handshake(io)
            .await?;
        tokio::task::spawn(async move {
            if let Err(err) = conn.await {
                println!("Connection failed: {:?}", err);
            }
        });

        let resp = sender.send_request(req).await?;
        Ok(resp.map(|b| b.boxed()))
    }
}

// Create a TCP connection to host:port, build a tunnel between the connection and
// the upgraded connection
#[tracing::instrument(name = "tunnel", ret, err)]
async fn tunnel(
    upgraded: Upgraded,
    addr: String,
    proxy_url: Arc<String>,
    proxy_authorization: HeaderValue,
) -> anyhow::Result<()> {
    // Connect to remote server
    let to_proxy_stream = TcpStream::connect(proxy_url.to_string()).await?;
    let (mut send_request, conn) = client::conn::http1::Builder::new()
        .handshake(TokioIo::new(to_proxy_stream))
        .await?;
    tokio::spawn(conn.with_upgrades());
    let req = Request::builder()
        .method(Method::CONNECT)
        // whatever
        .uri(addr.to_string())
        .header(header::UPGRADE, "")
        .header(header::PROXY_AUTHORIZATION, proxy_authorization)
        .body(empty())?;
    let res = send_request.send_request(req).await?;
    let stream = hyper::upgrade::on(res).await?;
    let mut to_proxy_stream_upgraded = TokioIo::new(stream);
    // let mut server = TcpStream::connect(addr).await?;
    let mut upgraded = TokioIo::new(upgraded);
    // Proxying data
    let (from_client, from_server) =
        tokio::io::copy_bidirectional(&mut upgraded, &mut to_proxy_stream_upgraded).await?;
    // Print message when done
    tracing::info!(
        "client wrote {} bytes and received {} bytes",
        from_client,
        from_server
    );
    Ok(())
}
