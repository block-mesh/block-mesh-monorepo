use crate::configuration::settings::Settings;
use crate::emails::email_client::EmailClient;
use crate::middlewares::authentication::{authentication_layer, Backend};
use crate::routes::twitter::context::Oauth2Ctx;
use crate::startup::routers::api_router::get_api_router;
use crate::startup::routers::leptos_router::get_leptos_router;
use crate::startup::routers::static_auth_router::get_static_auth_router;
use crate::startup::routers::static_un_auth_router::get_static_un_auth_router;
use crate::startup::routers::ws_router::get_ws_router;
use axum::{Extension, Router};
use axum_login::login_required;
use block_mesh_common::feature_flag_client::FlagValue;
use leptos::leptos_config::get_config_from_env;
use redis::aio::MultiplexedConnection;
use reqwest::Client;
use sqlx::postgres::PgPool;
use std::collections::HashMap;
use std::env;
use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;

use crate::worker::db_cleaner_cron::EnrichIp;
use crate::ws::connection_manager::ConnectionManager;
use block_mesh_common::env::app_env_var::AppEnvVar;
use block_mesh_common::env::env_var;
use block_mesh_common::env::get_env_var_or_panic::get_env_var_or_panic;
use block_mesh_common::interfaces::db_messages::{
    AggregateMessage, AnalyticsMessage, DailyStatMessage, UsersIpMessage,
};
use flume::Sender;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tower_http::timeout::TimeoutLayer;
use twitter_v2::authorization::Oauth2Client;

pub struct Application {
    app: Router,
    listener: TcpListener,
}

pub struct AppState {
    pub pool: PgPool,
    pub email_client: Arc<EmailClient>,
    pub client: Client,
    pub tx: Sender<JoinHandle<()>>,
    pub tx_daily_stat_agg: Sender<DailyStatMessage>,
    pub tx_analytics_agg: Sender<AnalyticsMessage>,
    pub flags: HashMap<String, FlagValue>,
    pub cleaner_tx: Sender<EnrichIp>,
    pub redis: MultiplexedConnection,
    pub ws_connection_manager: ConnectionManager,
    pub tx_users_ip_agg: Sender<UsersIpMessage>,
    pub tx_aggregate_agg: Sender<AggregateMessage>,
}

#[derive(Clone)]
pub struct ApplicationBaseUrl(pub String);

impl ApplicationBaseUrl {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Application {
    pub async fn build(settings: Settings, app_state: Arc<AppState>, db_pool: PgPool) -> Self {
        let governor_conf = Arc::new(
            GovernorConfigBuilder::default()
                .per_second(
                    env::var("REQUEST_PER_SECOND")
                        .unwrap_or("10".to_string())
                        .parse()
                        .unwrap_or(10),
                )
                .burst_size(
                    env::var("REQUEST_PER_SECOND_BURST")
                        .unwrap_or("30".to_string())
                        .parse()
                        .unwrap_or(30),
                )
                .finish()
                .unwrap(),
        );
        let governor_limiter = governor_conf.limiter().clone();
        let interval = Duration::from_secs(60);
        // a separate background task to clean up
        std::thread::spawn(move || loop {
            std::thread::sleep(interval);
            tracing::info!("rate limiting storage size: {}", governor_limiter.len());
            governor_limiter.retain_recent();
        });

        let auth_layer = authentication_layer(&db_pool, &app_state.redis).await;

        let app_env = get_env_var_or_panic(AppEnvVar::AppEnvironment);
        let app_env = <env_var::EnvVar as AsRef<String>>::as_ref(&app_env);
        let cors = match app_env.as_str() {
            "local" => CorsLayer::very_permissive(),
            _ => CorsLayer::permissive(),
        };

        let ws_router = get_ws_router();
        let auth_router = get_static_auth_router();
        let api_router = get_api_router();
        let un_auth_router = get_static_un_auth_router();
        let leptos_config = get_config_from_env().unwrap();
        let leptos_options = leptos_config.leptos_options;

        let path = Path::new("")
            .join(leptos_options.site_root.clone())
            .join(leptos_options.site_pkg_dir.clone());
        let leptos_pkg: Router<()> = Router::new().nest_service(
            &format!("/{}", leptos_options.site_pkg_dir),
            ServeDir::new(path),
        );

        let leptos_router = get_leptos_router();

        let oauth_ctx = Oauth2Ctx {
            client: Oauth2Client::new(
                env::var("TWITTER_CLIENT_ID").expect("could not find TWITTER_CLIENT_ID"),
                env::var("TWITTER_CLIENT_SECRET").expect("could not find TWITTER_CLIENT_SECRET"),
                env::var("TWITTER_CALLBACK_URL")
                    .expect("could not find TWITTER_CALLBACK_URL")
                    .parse()
                    .unwrap(),
            ),
        };

        let application_base_url = ApplicationBaseUrl(settings.application.base_url.clone());
        let backend = Router::new()
            .nest("/", auth_router)
            .route_layer(login_required!(Backend, login_url = "/login"))
            .nest("/", ws_router)
            .nest("/api", api_router)
            .nest("/", un_auth_router)
            .layer(Extension(application_base_url))
            .layer(Extension(db_pool.clone()))
            .layer(cors)
            .layer(auth_layer.clone())
            .with_state(app_state.clone());
        let app = Router::new()
            .layer(TimeoutLayer::new(Duration::from_millis(
                env::var("REQUEST_TIMEOUT")
                    .unwrap_or("3500".to_string())
                    .parse()
                    .unwrap_or(3500),
            )))
            .layer(GovernorLayer {
                config: governor_conf,
            })
            .nest("/", leptos_router)
            .nest("/", backend)
            .nest("/", leptos_pkg)
            .layer(Extension(Arc::new(Mutex::new(oauth_ctx))))
            .layer(auth_layer);

        let listener = TcpListener::bind(settings.application.address())
            .await
            .unwrap();
        tracing::info!("Listening on {}", listener.local_addr().unwrap());
        Application { app, listener }
    }

    pub async fn run(self) -> std::io::Result<()> {
        axum::serve(
            self.listener,
            self.app.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await
    }

    pub fn address(&self) -> String {
        format!("{}", self.listener.local_addr().unwrap())
    }

    pub fn port(&self) -> u16 {
        self.listener.local_addr().unwrap().port()
    }
}
