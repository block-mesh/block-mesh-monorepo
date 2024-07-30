use crate::helpers::{login, report_uptime, submit_bandwidth, task_poller};
use block_mesh_common::cli::CliOpts;
use block_mesh_common::constants::DeviceType;
use block_mesh_common::interfaces::server_api::LoginForm;
use clap::Parser;
use logger_general::tracing::setup_tracing;
use std::process::ExitCode;
use std::sync::Arc;
use std::time::Duration;

mod helpers;

#[tokio::main]
pub async fn main() -> anyhow::Result<ExitCode> {
    let args = CliOpts::parse();
    let api_token = login(LoginForm {
        email: args.email.clone(),
        password: args.password.clone(),
    })
    .await?;
    setup_tracing(api_token, DeviceType::Cli);

    let email = Arc::new(args.email);
    let api_token = Arc::new(api_token.to_string());

    let e = email.clone();
    let a = api_token.clone();
    let task_poller = tokio::spawn(async move {
        loop {
            let _ = task_poller(e.as_ref(), a.as_ref()).await;
            tokio::time::sleep(Duration::from_secs(30)).await;
        }
    });
    let e = email.clone();
    let a = api_token.clone();
    let uptime_poller = tokio::spawn(async move {
        loop {
            let _ = report_uptime(e.as_ref(), a.as_ref()).await;
            tokio::time::sleep(Duration::from_secs(30)).await;
        }
    });
    let e = email.clone();
    let a = api_token.clone();
    let bandwidth_poller = tokio::spawn(async move {
        loop {
            let _ = submit_bandwidth(e.as_ref(), a.as_ref()).await;
            tokio::time::sleep(Duration::from_secs(30)).await;
        }
    });

    tokio::select! {
        o = task_poller => tracing::error!("task_poller failed {:?}", o),
        o = uptime_poller => tracing::error!("uptime_poller failed {:?}", o),
        o = bandwidth_poller => tracing::error!("bandwidth_poller failed {:?}", o)
    };

    Ok(ExitCode::SUCCESS)
}
