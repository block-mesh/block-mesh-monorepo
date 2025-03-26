mod errors;
mod routes;

use crate::routes::get_router;
use axum::Router;
use block_mesh_common::env::environment::Environment;
use block_mesh_common::env::load_dotenv::load_dotenv;
use block_mesh_common::solana::get_keypair;
use chrono::{DateTime, Utc};
use database_utils::utils::connection::write_pool::write_pool;
use database_utils::utils::migrate::migrate;
use database_utils::utils::option_uuid::OptionUuid;
use logger_general::tracing::{get_sentry_layer, setup_tracing_stdout_only_with_sentry};
use serde::{Deserialize, Serialize};
use solana_sdk::signature::Keypair;
use sqlx::{PgPool, Postgres, Transaction};
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use std::{env, mem, process};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use uuid::Uuid;

pub async fn run_server(listener: TcpListener, app: Router<()>) -> std::io::Result<()> {
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
}

fn main() {
    let sentry_layer = get_sentry_layer();
    let sentry_url = env::var("SENTRY_ID").unwrap_or_default();
    let sentry_sample_rate = env::var("SENTRY_SAMPLE_RATE")
        .unwrap_or("0.1".to_string())
        .parse()
        .unwrap_or(0.1);
    if sentry_layer {
        let _guard = sentry::init((
            sentry_url,
            sentry::ClientOptions {
                debug: env::var("APP_ENVIRONMENT").unwrap_or_default() == "local",
                sample_rate: sentry_sample_rate,
                traces_sample_rate: sentry_sample_rate,
                release: sentry::release_name!(),
                ..Default::default()
            },
        ));
        mem::forget(_guard);
    }

    let _ = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async { run().await });
    tracing::error!("block mesh manager worker stopped, exiting with exit code 1");
    process::exit(1);
}

#[derive(Clone)]
pub struct IdAppState {
    pub db_pool: PgPool,
    pub environment: Environment,
    pub ext_keypair: Arc<Keypair>,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct Id {
    pub id: Uuid,
    pub email: String,
    pub api_token: String,
    pub fp: String,
    pub ip: String,
    pub created_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct IdTmp {
    pub id: OptionUuid,
    pub email: Option<String>,
    pub api_token: Option<String>,
    pub fp: Option<String>,
    pub ip: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

pub async fn get_or_create_id(
    transaction: &mut Transaction<'_, Postgres>,
    email: &str,
    api_token: &str,
    fp: &str,
    ip: &str,
) -> anyhow::Result<Id> {
    let uuid = Uuid::new_v4();
    let now = Utc::now();
    let idtmp = sqlx::query_as!(
        IdTmp,
        r#"
WITH extant AS (
	SELECT id, email, api_token, fp, ip, created_at
	FROM ids
	WHERE (email) = ($2)
),
inserted AS (
INSERT INTO ids ( id, email, api_token, fp, ip, created_at)
SELECT $1, $2, $3, $4 , $5 , $6
WHERE
	NOT EXISTS (SELECT	FROM extant)
	RETURNING  id, email, api_token, fp, ip, created_at
)
SELECT id, email, api_token, fp, ip, created_at
FROM inserted
UNION ALL
SELECT id, email, api_token, fp, ip, created_at
FROM extant;
"#,
        uuid,
        email,
        api_token,
        fp,
        ip,
        now
    )
    .fetch_one(&mut **transaction)
    .await?;
    let id = Id {
        id: idtmp.id.expect("MISSING ID"),
        email: idtmp.email.expect("MISSING EMAIL"),
        api_token: idtmp.api_token.expect("MISSING API TOKEN"),
        fp: idtmp.fp.expect("MISSING FP"),
        ip: idtmp.ip.expect("MISSING IP"),
        created_at: idtmp.created_at.expect("MISSING TIMESTAMP CREATED_AT"),
    };
    Ok(id)
}

impl IdAppState {
    pub async fn new() -> Self {
        let environment = env::var("APP_ENVIRONMENT").unwrap();
        let ext_keypair = get_keypair().unwrap();
        let environment = Environment::from_str(&environment).unwrap();
        let db_pool = write_pool(Some("DATABASE_URL".to_string())).await;
        Self {
            ext_keypair: Arc::new(ext_keypair),
            environment,
            db_pool,
        }
    }
}

#[tracing::instrument(name = "run", skip_all, ret, err)]
async fn run() -> anyhow::Result<()> {
    load_dotenv();
    setup_tracing_stdout_only_with_sentry();
    tracing::info!("Starting worker");
    let state = IdAppState::new().await;
    let env = env::var("APP_ENVIRONMENT").expect("APP_ENVIRONMENT is not set");
    migrate(&state.db_pool, env)
        .await
        .expect("Failed to migrate database");
    let router = get_router(state);
    let cors = CorsLayer::permissive();
    let app = Router::new().nest("/", router).layer(cors);
    let port = env::var("PORT").unwrap_or("8009".to_string());
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    tracing::info!("Listening on {}", listener.local_addr()?);
    let server_task = run_server(listener, app);
    tokio::select! {
        o = server_task => panic!("server task exit {:?}", o)
    }
}
