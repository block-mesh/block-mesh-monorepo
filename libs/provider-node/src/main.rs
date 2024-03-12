use axum::{body::Body, extract::Request, http::Method, routing::get, Router};
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper_util::rt::TokioIo;
use provider_node::app_state::AppState;
use provider_node::proxy_server::proxy::proxy;
use provider_node::routes::health_check::health_check;
use provider_node::solana::manager::SolanaManager;
use provider_node::token_management::channels::{
    update_token_manager, ChannelMessage, TokenManagerHashMap,
};
use rustc_hash::FxHashMap;
use solana_sdk::signature::Signer;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::fs::try_exists;
use tokio::net::TcpListener;
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
        .with(tracing_subscriber::fmt::layer())
        .init();

    let mut solana_manager = SolanaManager::new().await.unwrap();
    solana_manager
        .create_provider_account_if_needed()
        .await
        .unwrap();

    let mut token_manager: TokenManagerHashMap = FxHashMap::default();
    let (tx, mut rx) = broadcast::channel::<ChannelMessage>(16);

    tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            update_token_manager(&msg, &mut token_manager).await;
        }
    });

    let app_state = Arc::new(AppState { tx });
    let router_svc = Router::new()
        .route("/health_check", get(health_check))
        .route("/", get(|| async { "OK" }))
        .with_state(app_state.clone());

    let tower_service = tower::service_fn(move |req: Request<_>| {
        let app_state = app_state.clone();
        let router_svc = router_svc.clone();
        let req = req.map(Body::new);
        async move {
            if req.method() == Method::CONNECT {
                proxy(app_state, req).await
            } else {
                router_svc.oneshot(req).await.map_err(|err| match err {})
            }
        }
    });

    let hyper_service = hyper::service::service_fn(move |request: Request<Incoming>| {
        tower_service.clone().call(request)
    });

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    loop {
        let (stream, _) = listener.accept().await.unwrap();
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
    }
}
