pub mod app_state;
pub mod client_server;
pub mod ip_getter;
pub mod proxy_server;
pub mod routes;
pub mod token_management;

use app_state::AppState;
use block_mesh_common::cli::ProxyMasterNodeOptions;
// use block_mesh_solana_client::manager::SolanaManager;
use client_server::clients_endpoint::listen_for_clients_connecting;
use futures_util::future::join_all;
use ip_getter::get_ip;
use proxy_server::proxy_endpoint::listen_for_proxies_connecting;
use proxy_server::proxy_pool::ProxyPool;
use rustc_hash::FxHashMap;
use std::net::SocketAddr;
use std::process::ExitCode;
use std::sync::Arc;
use token_management::channels::{update_token_manager, ChannelMessage, TokenManagerHashMap};
use tokio::net::TcpListener;
use tokio::sync::broadcast;

#[tracing::instrument(name = "proxy_master_main", ret, err)]
pub async fn proxy_master_main(
    proxy_master_node_options: &ProxyMasterNodeOptions,
) -> anyhow::Result<ExitCode> {
    let ip_addr = get_ip().await?;
    tracing::info!("Local IP address: {}", ip_addr);
    let pool = ProxyPool::default();
    let addr_proxies = SocketAddr::from(([0, 0, 0, 0], proxy_master_node_options.proxy_port));
    tracing::info!("Binding to proxy_port: {}", addr_proxies);
    let proxy_listener = TcpListener::bind(addr_proxies).await?;
    tracing::info!("Listening on for proxies on: {}", addr_proxies);
    let addr_clients = SocketAddr::from(([0, 0, 0, 0], proxy_master_node_options.client_port));
    tracing::info!("Binding to client_port: {}", addr_clients);
    let client_listener = TcpListener::bind(addr_clients).await?;
    tracing::info!("Listening on for clients on: {}", addr_clients);

    // let ip_addr = match ip_addr {
    //     IpAddr::V4(ip) => {
    //         tracing::info!("IP address: {}", ip);
    //         ip
    //     }
    //     _ => {
    //         tracing::error!("IP address is not IPv4");
    //         exit(1);
    //     }
    // };
    // let mut solana_manager = SolanaManager::new(
    //     &proxy_master_node_options.keypair_path,
    //     &proxy_master_node_options.program_id,
    // )
    // .await?;
    // solana_manager
    //     .create_or_update_provider_node_if_needed(
    //         ip_addr,
    //         proxy_master_node_options.proxy_port,
    //         proxy_master_node_options.client_port,
    //     )
    //     .await?;
    //
    // let solana_manager = Arc::new(tokio::sync::RwLock::new(solana_manager));
    let token_manager: TokenManagerHashMap = FxHashMap::default();
    let token_manager = Arc::new(tokio::sync::RwLock::new(token_manager));
    let (tx, mut rx) = broadcast::channel::<ChannelMessage>(16);

    let tkn = token_manager.clone();
    tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            let token_manager = tkn.clone();
            update_token_manager(&msg, token_manager).await;
        }
    });

    let app_state = Arc::new(AppState {
        tx,
        token_manager,
        // solana_manager,
    });

    // let clients_router = Router::new()
    //     .route("/health_check", get(routes::health_check::handler))
    //     .with_state(app_state.clone());
    //
    // let proxy_router = Router::new()
    //     .route("/health_check", get(routes::health_check::handler))
    //     .with_state(app_state.clone());
    //
    // let router_svc = Router::new()
    //     .route("/health_check", get(routes::health_check::handler))
    //     .route("/register", post(routes::register_client::handler))
    //     .route("/", get(|| async { "OK" }))
    //     .with_state(app_state.clone());
    //
    // let tower_service = tower::service_fn(move |req: Request<_>| {
    //     let app_state = app_state.clone();
    //     let router_svc = router_svc.clone();
    //     let req = req.map(Body::new);
    //     async move {
    //         if req.method() == Method::CONNECT {
    //             proxy(app_state, req).await
    //         } else {
    //             router_svc.oneshot(req).await.map_err(|err| match err {})
    //         }
    //     }
    // });
    //
    // let hyper_service = hyper::service::service_fn(move |request: Request<Incoming>| {
    //     tower_service.clone().call(request)
    // });
    //
    // let addr = SocketAddr::from(([0, 0, 0, 0], proxy_master_node_cli_args.port));
    // tracing::debug!("listening on {}", addr);
    //
    // let listener = TcpListener::bind(addr).await.unwrap();
    // loop {
    //     let (stream, _) = listener.accept().await.unwrap();
    //     let io = TokioIo::new(stream);
    //     let hyper_service = hyper_service.clone();
    //     tokio::task::spawn(async move {
    //         if let Err(err) = http1::Builder::new()
    //             .preserve_header_case(true)
    //             .title_case_headers(true)
    //             .serve_connection(io, hyper_service)
    //             .with_upgrades()
    //             .await
    //         {
    //             tracing::error!("Failed to serve connection: {:?}", err);
    //         }
    //     });
    // }

    let proxy_listener_pool = pool.clone();
    let proxy_app_state = app_state.clone();
    let proxy_listener_task = tokio::task::spawn(async move {
        listen_for_proxies_connecting(proxy_listener_pool, proxy_listener, proxy_app_state).await
    });
    let proxy_listener_pool = pool.clone();
    let client_app_state = app_state.clone();
    let clients_listener_task = tokio::task::spawn(async move {
        listen_for_clients_connecting(proxy_listener_pool, client_listener, client_app_state).await;
    });
    let _ = join_all(vec![proxy_listener_task, clients_listener_task]).await;
    Ok(ExitCode::SUCCESS)
}
