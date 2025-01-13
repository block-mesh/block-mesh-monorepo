mod ws_client;

use crate::ws_client::run_client;
use clap::Parser;
use database_utils::utils::connection::write_pool::write_pool;
use database_utils::utils::health_check::health_check;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use libc::c_int;
use serde::{Deserialize, Serialize};
use signal_hook::consts::signal::*;
use signal_hook::iterator::Signals;
use signal_hook::low_level;
use std::time::Duration;
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::sync::{broadcast, OnceCell};
use tokio::time::sleep;

#[derive(Parser, Clone, Serialize, Deserialize)]
pub struct Options {
    #[clap(long)]
    pub mode: String,
    #[clap(long)]
    pub num_clients: Option<usize>,
}

pub static SIGNAL_TX: OnceCell<Sender<()>> = OnceCell::const_new();
pub static SIGNAL_RX: OnceCell<Receiver<()>> = OnceCell::const_new();

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Options::parse();
    let (tx, rx) = broadcast::channel(16);
    SIGNAL_TX.get_or_init(|| async { tx.clone() }).await;
    SIGNAL_RX.get_or_init(|| async { rx }).await;
    tokio::spawn(async move {
        const SIGNALS: &[c_int] = &[
            SIGTERM, SIGQUIT, SIGINT, SIGTSTP, SIGWINCH, SIGHUP, SIGCHLD, SIGCONT,
        ];
        if let Ok(mut sigs) = Signals::new(SIGNALS) {
            for signal in &mut sigs {
                eprintln!("Received signal {:?}", signal);
                let _ = tx.send(());
                sleep(Duration::from_millis(1_000)).await;
                eprintln!("Send");
                // After printing it, do whatever the signal was supposed to do in the first place
                let _ = low_level::emulate_default_handler(signal);
            }
        }
    });

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
