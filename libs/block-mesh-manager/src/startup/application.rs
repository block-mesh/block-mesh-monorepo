use crate::configuration::settings::Settings;
use crate::middlewares::authentication::{authentication_layer, Backend};
use crate::routes::twitter::context::Oauth2Ctx;
use crate::startup::routers::api_router::get_api_router;
//use crate::startup::routers::leptos_router::get_leptos_router;
use crate::startup::routers::static_auth_router::get_static_auth_router;
use crate::startup::routers::static_un_auth_router::get_static_un_auth_router;
use axum::extract::Request;
use axum::{Extension, Router};
use axum_login::login_required;
use block_mesh_common::constants::DeviceType;
use block_mesh_common::email_client::client::EmailClient;
use block_mesh_common::env::app_env_var::AppEnvVar;
use block_mesh_common::env::env_var;
use block_mesh_common::env::get_env_var_or_panic::get_env_var_or_panic;
use block_mesh_common::feature_flag_client::FlagValue;
use block_mesh_common::interfaces::server_api::{CheckTokenResponseMap, GetTokenResponseMap};
use dash_with_expiry::hash_map_with_expiry::HashMapWithExpiry;
use dash_with_expiry::hash_set_with_expiry::HashSetWithExpiry;
use http::{header, HeaderValue};
//use leptos_config::get_config_from_env;
use logger_general::tracing::get_sentry_layer;
use redis::aio::MultiplexedConnection;
use reqwest::Client;
use sentry_tower::NewSentryLayer;
use sqlx::postgres::PgPool;
use std::collections::HashMap;
use std::env;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::timeout::TimeoutLayer;
use twitter_v2::authorization::Oauth2Client;

pub struct Application {
    app: Router,
    listener: TcpListener,
}

pub struct AppState {
    pub wallet_login_nonce: HashMapWithExpiry<String, String>,
    pub rate_limiter: HashSetWithExpiry<String>,
    pub enable_hcaptcha: bool,
    pub enable_recaptcha: bool,
    pub enable_proof_of_humanity: bool,
    pub hcaptcha_site_key: String,
    pub hcaptcha_secret_key: String,
    pub recaptcha_site_key_v2: String,
    pub recaptcha_secret_key_v2: String,
    pub recaptcha_site_key_v3: String,
    pub recaptcha_secret_key_v3: String,
    pub cf_enforce: bool,
    pub cf_secret_key: String,
    pub cf_site_key: String,
    pub task_limit: bool,
    pub rate_limit: bool,
    pub submit_bandwidth_limit: bool,
    pub get_token_map: GetTokenResponseMap,
    pub check_token_map: CheckTokenResponseMap,
    pub invite_codes: HashMapWithExpiry<String, String>,
    pub wallet_addresses: HashMapWithExpiry<String, Option<String>>,
    pub pool: PgPool,
    pub follower_pool: PgPool,
    pub channel_pool: PgPool,
    pub email_client: Arc<EmailClient>,
    pub client: Client,
    pub flags: Arc<RwLock<HashMap<String, FlagValue>>>,
    pub redis: MultiplexedConnection,
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
        let auth_layer = authentication_layer(&db_pool, &app_state.redis).await;

        let app_env = get_env_var_or_panic(AppEnvVar::AppEnvironment);
        let app_env = <env_var::EnvVar as AsRef<String>>::as_ref(&app_env);
        let cors = CorsLayer::very_permissive();

        let auth_router = get_static_auth_router();
        let api_router = get_api_router();
        let un_auth_router = get_static_un_auth_router();
        //let leptos_config = get_config_from_env().unwrap();
        //let leptos_options = leptos_config.leptos_options;

        //let path = Path::new("")
        //    .join(leptos_options.site_root.clone())
        //    .join(leptos_options.site_pkg_dir.clone());
        //let leptos_pkg: Router<()> = Router::new().nest_service(
        //    &format!("/{}", leptos_options.site_pkg_dir),
        //    ServeDir::new(path),
        //);

        //let leptos_router = get_leptos_router();

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

        let sentry_layer = get_sentry_layer();
        let backend = Router::new()
            .nest("/", auth_router)
            .route_layer(login_required!(Backend, login_url = "/login"))
            .nest("/api", api_router.clone())
            .nest(
                &format!("/{}/api", DeviceType::Extension),
                api_router.clone(),
            )
            .nest(&format!("/{}/api", DeviceType::Cli), api_router.clone())
            .nest(&format!("/{}/api", DeviceType::Worker), api_router.clone())
            .nest(
                &format!("/{}/api", DeviceType::AppServer),
                api_router.clone(),
            )
            .nest(&format!("/{}/api", DeviceType::Unknown), api_router.clone())
            .nest("/", un_auth_router);

        let backend = backend
            .layer(Extension(application_base_url))
            .layer(Extension(db_pool.clone()))
            .layer(cors)
            .layer(auth_layer.clone())
            // .layer(TraceLayer::new_for_http()
            //     .make_span_with(|request: &Request<Body>| {
            //     tracing::info_span!("request", method = %request.method(), uri = %request.uri())
            //     })
            //     .on_response(|response: &Response<_>, latency: Duration, _span: &Span| {
            //         tracing::info!("Response status = {}, latency = {}ms", &response.status().as_u16(), latency.as_millis());
            //     }))
            .with_state(app_state.clone());
        #[allow(deprecated)]
        let backend = if sentry_layer {
            backend
                .layer(NewSentryLayer::<Request>::new_from_top())
                .layer(sentry_tower::SentryHttpLayer::with_transaction())
        } else {
            backend
        };

        let app = Router::new();

        let timeout_layer = env::var("TIMEOUT_LAYER")
            .unwrap_or("false".to_string())
            .parse()
            .unwrap_or(false);
        let app = if timeout_layer {
            app.layer(TimeoutLayer::new(Duration::from_millis(
                env::var("REQUEST_TIMEOUT")
                    .unwrap_or("3500".to_string())
                    .parse()
                    .unwrap_or(3500),
            )))
        } else {
            app
        };
        let permissions = "'self' 'unsafe-eval' 'unsafe-inline' 'wasm-unsafe-eval' data: https://fonts.gstatic.com https://fonts.googleapis.com https://rsms.me https://opencollective.com https://cdn.jsdelivr.net https://*.cloudflare.com https://*.blockmesh.xyz https://*.perceptrons.xyz https://*.googletagmanager.com https://r2-images.blockmesh.xyz https://imagedelivery.net https://*.google-analytics.com chrome-extension://obfhoiefijlolgdmphcekifedagnkfjp ".to_string();
        let permissions = format!("{} {} ", permissions, "https://*.google.com https://*.hcaptcha.com https://*.cloudflareinsights.com https://*.hcaptcha.com https://*.gstatic.com https://*.cloudflare.com");
        let permissions = format!(
            "{} {} ",
            permissions,
            env::var("EXTRA_CSP").unwrap_or_default()
        );
        let permissions = if app_env == "local" {
            format!("{} chrome-extension://*", permissions)
        } else {
            permissions
        };
        let mut csp = String::default();
        csp.push_str("default-src 'self' ;");
        csp.push_str(&format!("object-src {} ;", permissions));
        csp.push_str(&format!("style-src {} ;", permissions));
        csp.push_str(&format!("script-src-elem {} ;", permissions));
        csp.push_str(&format!("script-src {} ;", permissions));
        csp.push_str(&format!("img-src {} ;", permissions));
        csp.push_str(&format!(
            "connect-src {} ;",
            permissions
                .replace("'unsafe-eval'", "")
                .replace("'unsafe-inline'", "")
        ));
        csp.push_str(&format!("font-src {} ;", permissions));
        csp.push_str(&format!("frame-src {} ;", permissions));
        csp.push_str(&format!(
            "frame-ancestors {} ;",
            permissions
                .replace("'unsafe-eval'", "")
                .replace("'unsafe-inline'", "")
        ));
        csp.push_str(&format!("child-src {} ;", permissions));
        csp.push_str(&format!("worker-src {} ;", permissions));
        let csp_header = HeaderValue::from_str(&csp).unwrap();
        let enforce_csp = env::var("ENFORCE_CSP")
            .unwrap_or("false".to_string())
            .parse()
            .unwrap_or(false);
        let app = app
            //.nest("/", leptos_router)
            .nest("/", backend)
            //.nest("/", leptos_pkg)
            .layer(Extension(Arc::new(Mutex::new(oauth_ctx))))
            .layer(auth_layer);
        let app = if enforce_csp {
            app.layer(SetResponseHeaderLayer::overriding(
                header::CONTENT_SECURITY_POLICY,
                csp_header,
            ))
        } else {
            app
        };
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
