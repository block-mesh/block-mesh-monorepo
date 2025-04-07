use block_mesh_common::env::environment::Environment;
use block_mesh_common::solana::get_keypair;
use database_utils::utils::connection::write_pool::write_pool;
use rama::error::OpaqueError;
use solana_sdk::signature::Keypair;
use sqlx::PgPool;
use std::env;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Clone)]
pub struct RamaState {
    pub db_pool: PgPool,
    pub environment: Environment,
    pub ext_keypair: Arc<Keypair>,
}

impl RamaState {
    pub async fn new() -> Result<Self, OpaqueError> {
        let environment = env::var("APP_ENVIRONMENT").unwrap();
        let ext_keypair = get_keypair().unwrap();
        let environment = Environment::from_str(&environment).unwrap();
        let db_pool = write_pool(None).await;
        Ok(RamaState {
            environment,
            ext_keypair: Arc::new(ext_keypair),
            db_pool,
        })
    }
}
