mod ws_client;

use crate::ws_client::run_client;
use clap::Parser;
use database_utils::utils::connection::write_pool::write_pool;
use database_utils::utils::health_check::health_check;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, FixedOffset};

#[derive(Parser, Clone, Serialize, Deserialize)]
pub struct Options {
    #[clap(long)]
    pub mode: String,
    #[clap(long)]
    pub num_clients: Option<usize>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let created_at = "Fri Mar 08 15:31:08 +0000 2024".to_string();
    let format = "%a %b %d %H:%M:%S %z %Y";
    let created_at: DateTime<FixedOffset> = DateTime::parse_from_str(&created_at, format)?;
    println!("result = {}", created_at);
    let args = Options::parse();
    match args.mode.as_str() {
        "db" => {
            let pool = write_pool(None).await;
            let mut transaciton = create_txn(&pool).await?;
            health_check(&mut *transaciton).await?;
            commit_txn(transaciton).await?;
        }
        "ws" => {
            run_client(args.num_clients.unwrap_or(10)).await;
        }
        _ => {
            eprintln!("unsupported mode {}", args.mode);
        }
    }

    println!("Hello, world!");
    Ok(())
}
