use crate::configuration::settings::Settings;
use crate::emails::email_client::EmailClient;
use crate::envars::app_env_var::AppEnvVar;
use crate::envars::env_var;
use crate::envars::get_env_var_or_panic::get_env_var_or_panic;
use crate::middlewares::authentication::{authentication_layer, Backend};
use crate::startup::routers::api_router::get_api_router;
use crate::startup::routers::leptos_router::get_leptos_router;
use crate::startup::routers::static_auth_router::get_static_auth_router;
use crate::startup::routers::static_un_auth_router::get_static_un_auth_router;
use crate::worker::db_cleaner_cron::EnrichIp;
use axum::{Extension, Router};
use axum_login::login_required;
use leptos::leptos_config::get_config_from_env;
use reqwest::Client;
use sqlx::postgres::PgPool;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::mpsc::UnboundedSender;
use tokio::task::JoinHandle;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

pub struct Application {
    app: Router,
    listener: TcpListener,
}

pub struct AppState {
    pub pool: PgPool,
    pub email_client: Arc<EmailClient>,
    pub client: Client,
    pub tx: tokio::sync::mpsc::Sender<JoinHandle<()>>,
    pub flags: HashMap<String, bool>,
    pub cleaner_tx: UnboundedSender<EnrichIp>,
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
        let auth_layer = authentication_layer(&db_pool).await;

        let app_env = get_env_var_or_panic(AppEnvVar::AppEnvironment);
        let app_env = <env_var::EnvVar as AsRef<String>>::as_ref(&app_env);
        let cors = match app_env.as_str() {
            "local" => CorsLayer::very_permissive(),
            _ => CorsLayer::permissive(),
        };

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

        let application_base_url = ApplicationBaseUrl(settings.application.base_url.clone());
        let backend = Router::new()
            .nest("/", auth_router)
            .route_layer(login_required!(Backend, login_url = "/login"))
            .nest("/api", api_router)
            .nest("/", un_auth_router)
            .layer(Extension(application_base_url))
            .layer(Extension(db_pool.clone()))
            .layer(cors)
            .layer(auth_layer.clone())
            .with_state(app_state.clone());

        let app = Router::new()
            .nest("/", leptos_router)
            .route_layer(login_required!(Backend, login_url = "/login"))
            .nest("/", backend)
            .nest("/", leptos_pkg)
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
