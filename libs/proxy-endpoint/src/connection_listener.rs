// use crate::endpoint_headers::process_endpoint_headers;
use block_mesh_common::http::{empty, full, host_addr};
// use block_mesh_solana_client::manager::{EndpointNodeToProviderNodeHeader, SolanaManager};
use bytes::Bytes;
use http::header;
use http_body_util::combinators::BoxBody;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::upgrade::Upgraded;
use hyper::{client, http, Method, Request, Response};
use hyper_util::rt::TokioIo;
use std::net::{SocketAddr, ToSocketAddrs};
// use std::sync::Arc;
use tokio::net::TcpStream;

// #[tracing::instrument(name = "listen_for_proxies_connecting", skip(solana_manager), ret, err)]
#[tracing::instrument(name = "listen_for_proxies_connecting", ret, err)]
pub async fn listen_for_proxies_connecting(
    addr: SocketAddr,
    // auth_header: EndpointNodeToProviderNodeHeader,
    // solana_manager: Arc<SolanaManager>,
) -> anyhow::Result<()> {
    while let Ok(stream) = TcpStream::connect(addr).await {
        // let auth_header = auth_header.clone();
        // let solana_manager = solana_manager.clone();
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

        let auth_header = "{}";
        let req = Request::builder()
            .method(Method::CONNECT)
            // whatever
            .uri(addr.to_string())
            .header(
                header::PROXY_AUTHORIZATION,
                serde_json::to_string(&auth_header)?,
            )
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

// #[tracing::instrument(name = "proxy", skip(solana_manager), ret, err)]
#[tracing::instrument(name = "proxy", ret, err)]
async fn proxy(
    req: Request<hyper::body::Incoming>,
    // solana_manager: Arc<SolanaManager>,
) -> anyhow::Result<Response<BoxBody<Bytes, hyper::Error>>> {
    // let proxy_authorization = process_endpoint_headers(solana_manager.clone(), &mut req).await?;
    // let memos = proxy_authorization.prepare_for_memo();
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
                        match tunnel(upgraded, addr).await {
                            Ok(_) => {
                                // TODO : send memo here
                                // if let Err(e) = solana_manager.send_memos(memos).await {
                                //     tracing::error!("send memo error: {}", e);
                                // }
                                tracing::info!("tunnel success");
                            }
                            Err(e) => tracing::error!("server io error: {}", e),
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
        Ok(Response::new(empty()))
        //tracing::info!("NOT CONNECT request");
        //let host = req.uri().host().expect("uri has no host");
        //let port = req.uri().port_u16().unwrap_or(80);

        //let stream = TcpStream::connect((host, port)).await.unwrap();
        //let io = TokioIo::new(stream);

        //let (mut sender, conn) = Builder::new()
        //    .preserve_header_case(true)
        //    .title_case_headers(true)
        //    .handshake(io)
        //    .await?;
        //tokio::task::spawn(async move {
        //    if let Err(err) = conn.await {
        //        tracing::info!("Connection failed: {:?}", err);
        //    }
        //});

        //let resp = sender.send_request(req).await?;
        //Ok(resp.map(|b| b.boxed()))
    }
}

// Create a TCP connection to host:port, build a tunnel between the connection and
// the upgraded connection
#[tracing::instrument(name = "tunnel", ret, err)]
async fn tunnel(upgraded: Upgraded, addr: String) -> std::io::Result<()> {
    // TODO: replace to_socket_addrs with - https://crates.io/crates/hickory-resolver
    let addr = addr
        .to_socket_addrs()?
        .find(|a| a.is_ipv4())
        .expect("No IPv4 address found");
    let mut server = TcpStream::connect(addr).await?;
    tracing::info!(
        "tunnel local address: {:?} | addr: {:?}",
        server.local_addr()?,
        addr
    );
    let mut upgraded = TokioIo::new(upgraded);
    let (from_client, from_server) =
        tokio::io::copy_bidirectional(&mut upgraded, &mut server).await?;
    tracing::info!(from_client, from_server);
    Ok(())
}
