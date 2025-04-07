//! entrypoint for rama-cli

#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]
#![cfg_attr(test, allow(clippy::float_cmp))]
#![cfg_attr(not(test), warn(clippy::print_stdout, clippy::dbg_macro))]

use clap::Parser;
use rama::error::BoxError;

pub mod cmd;
use crate::cmd::fp::CliCommand;
use crate::cmd::fp::rama_state::RamaState;
use cmd::fp;

pub mod error;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = CliCommand::parse();
    let state = RamaState::new().await?;
    fp::run(cfg, state).await?;
    Ok(())
}
