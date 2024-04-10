use crate::app_state::AppState;
use crate::proxy_server::proxy_pool::ProxyPool;
use crate::token_management::client_headers::process_client_headers;
use block_mesh_common::http::empty;
use bytes::Bytes;
use http_body_util::combinators::BoxBody;
use hyper::service::service_fn;
use hyper::{client, server, Method, Request, Response};
use hyper_util::rt::TokioIo;
use std::sync::Arc;
use tokio::io::copy_bidirectional;
use tokio::net::TcpListener;

#[tracing::instrument(name = "listen_for_clients_connecting", skip(app_state))]
pub async fn listen_for_clients_connecting(
    pool: ProxyPool,
    client_listener: TcpListener,
    app_state: Arc<AppState>,
) {
    while let Ok((stream, addr)) = client_listener.accept().await {
        let pool = pool.clone();
        let app_state = app_state.clone();
        tokio::spawn(async move {
            let app_state = app_state.clone();
            if let Err(err) = server::conn::http1::Builder::new()
                .preserve_header_case(true)
                .title_case_headers(true)
                .serve_connection(
                    TokioIo::new(stream),
                    service_fn(move |req| {
                        handle_client_request(pool.clone(), req, app_state.clone())
                    }),
                )
                .with_upgrades()
                .await
            {
                tracing::error!("Failed to serve connection from addr {:?}: {:?}", addr, err);
            }
        });
    }
}

#[tracing::instrument(name = "handle_client_request", skip(app_state), ret, err)]
async fn handle_client_request(
    pool: ProxyPool,
    mut req: Request<hyper::body::Incoming>,
    app_state: Arc<AppState>,
) -> anyhow::Result<Response<BoxBody<Bytes, hyper::Error>>> {
    process_client_headers(app_state, &mut req).await?;

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
        tokio::spawn(async move {
            match hyper::upgrade::on(&mut req).await {
                // TODO: can add headers here
                Ok(upgraded) => {
                    let proxy = pool.get().await.unwrap();
                    let (mut send_request, conn) =
                        client::conn::http1::Builder::new().handshake(proxy).await?;
                    tokio::spawn(conn.with_upgrades());
                    let res = send_request.send_request(req).await?;
                    let stream = hyper::upgrade::on(res).await?;
                    let (from_client, from_server) =
                        copy_bidirectional(&mut TokioIo::new(upgraded), &mut TokioIo::new(stream))
                            .await
                            .unwrap();
                    tracing::info!(from_client, from_server);
                }
                Err(e) => tracing::error!("upgrade error = {}", e),
            }
            Ok::<(), hyper::Error>(())
        });
        Ok(Response::new(empty()))
    } else {
        // TODO : Process request - can register client here
        Ok(Response::new(empty()))
    }
}
