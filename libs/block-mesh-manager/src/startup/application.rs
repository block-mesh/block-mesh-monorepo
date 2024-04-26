use crate::configuration::settings::Settings;
use crate::envars::app_env_var::AppEnvVar;
use crate::envars::env_var;
use crate::envars::get_env_var_or_panic::get_env_var_or_panic;
use crate::middlewares::authentication::{authentication_layer, Backend};
use crate::routes;
use axum::routing::{get, post};
use axum::{Extension, Router};
use axum_login::login_required;
use sqlx::postgres::PgPool;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

pub struct Application {
    app: Router,
    listener: TcpListener,
}

pub struct AppState {
    pub pool: PgPool,
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

        let auth_router = Router::new()
            .route("/logout", get(routes::logout::get::handler))
            .route(
                "/create_task",
                get(routes::tasks::create_task::handler)
                    .post(routes::tasks::create_task_post::handler),
            )
            .route("/tasks_table", get(routes::tasks::tasks_table::handler))
            .route("/dashboard", get(routes::dashboard::get::handler));

        let api_router = Router::new()
            .route("/get_token", post(routes::api_token::get_token::handler))
            .route("/get_task", post(routes::tasks::get_task::handler))
            .route("/submit_task", post(routes::tasks::submit_task::handler))
            .route(
                "/create_task_with_token",
                post(routes::tasks::create_task_with_token::handler),
            );

        let un_auth_router = Router::new()
            .route(
                "/",
                get(routes::login::login_form::handler).post(routes::login::login_post::handler),
            )
            .route(
                "/login",
                get(routes::login::login_form::handler).post(routes::login::login_post::handler),
            )
            .route(
                "/register",
                get(routes::register::register_form::handler)
                    .post(routes::register::register_post::handler),
            )
            .route(
                "/api/check_token",
                post(routes::api_token::check_token::handler),
            )
            .route("/health_check", get(routes::health_check::get::handler));

        let application_base_url = ApplicationBaseUrl(settings.application.base_url.clone());
        let app = Router::new()
            .nest("/", auth_router)
            .route_layer(login_required!(Backend, login_url = "/login"))
            .nest("/api", api_router)
            .nest("/", un_auth_router)
            .layer(Extension(application_base_url))
            .layer(Extension(db_pool.clone()))
            .layer(cors)
            .layer(auth_layer)
            .with_state(app_state.clone());

        let listener = TcpListener::bind(settings.application.address())
            .await
            .unwrap();
        tracing::info!("Listening on {}", listener.local_addr().unwrap());
        Application { app, listener }
    }

    pub async fn run(self) -> std::io::Result<()> {
        axum::serve(self.listener, self.app).await
    }

    pub fn address(&self) -> String {
        format!("{}", self.listener.local_addr().unwrap())
    }

    pub fn port(&self) -> u16 {
        self.listener.local_addr().unwrap().port()
    }
}
