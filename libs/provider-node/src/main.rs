use axum::routing::post;
use axum::{body::Body, extract::Request, http::Method, routing::get, Router};
use block_mesh_solana_client::manager::SolanaManager;
use clap::Parser;
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper_util::rt::TokioIo;
use provider_node::app_state::AppState;
use provider_node::cli_args::ProviderNodeCliArgs;
use provider_node::ip_getter::get_ip;
use provider_node::proxy_server::proxy::proxy;
use provider_node::routes;
use provider_node::token_management::channels::{
    update_token_manager, ChannelMessage, TokenManagerHashMap,
};
use rustc_hash::FxHashMap;
use std::net::{IpAddr, SocketAddr};
use std::process::exit;
use std::str::FromStr;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::signal;
use tokio::sync::broadcast;
use tower::Service;
use tower::ServiceExt;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "trace".into()),
        )
        .with(tracing_subscriber::fmt::layer().with_ansi(false))
        .init();
    let provider_node_cli_args = ProviderNodeCliArgs::parse();
    let ip_addr = get_ip().await.unwrap();
    tracing::info!("IP address: {}", ip_addr);

    let ip_addr = match ip_addr {
        IpAddr::V4(ip) => {
            tracing::info!("IP address: {}", ip);
            ip
        }
        _ => {
            tracing::error!("IP address is not IPv4");
            exit(1);
        }
    };

    let mut solana_manager = SolanaManager::new(
        &provider_node_cli_args.keypair_path,
        &provider_node_cli_args.program_id,
    )
    .await
    .unwrap();
    solana_manager
        .create_or_update_provider_node_if_needed(ip_addr, provider_node_cli_args.port)
        .await
        .unwrap();

    let solana_manager = Arc::new(tokio::sync::RwLock::new(solana_manager));
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
        solana_manager,
    });
    let router_svc = Router::new()
        .route("/health_check", get(routes::health_check::handler))
        .route("/register", post(routes::register_client::handler))
        .route("/", get(|| async { "OK" }))
        .with_state(app_state.clone());

    let tower_service = tower::service_fn(move |req: Request<_>| {
        println!("Request: {:?}", req);
        let app_state = app_state.clone();
        let router_svc = router_svc.clone();
        let req = req.map(Body::new);
        async move {
            if req.method() == Method::CONNECT {
                println!("CONNECT request");
                proxy(app_state, req).await
            } else {
                router_svc.oneshot(req).await.map_err(|err| match err {})
            }
        }
    });

    let hyper_service = hyper::service::service_fn(move |request: Request<Incoming>| {
        tower_service.clone().call(request)
    });
    // let addr = SocketAddr::from(([0, 0, 0, 0], provider_node_cli_args.port));
    let addr = SocketAddr::from_str(&provider_node_cli_args.proxy_manager_address).unwrap();
    tracing::debug!("listening on {}", addr);
    let stream = TcpStream::connect(addr).await.unwrap();
    let io = TokioIo::new(stream);
    let hyper_service = hyper_service.clone();
    tokio::task::spawn(async move {
        if let Err(err) = http1::Builder::new()
            .preserve_header_case(true)
            .title_case_headers(true)
            .serve_connection(io, hyper_service)
            .with_upgrades()
            .await
        {
            tracing::error!("Failed to serve connection: {:?}", err);
        }
    });
    signal::ctrl_c().await.expect("failed to listen for event");
}
