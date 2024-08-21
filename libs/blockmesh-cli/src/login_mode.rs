use crate::helpers::{login, report_uptime, submit_bandwidth, task_poller};
use block_mesh_common::constants::DeviceType;
use block_mesh_common::interfaces::server_api::LoginForm;
use logger_general::tracing::setup_tracing;
use std::process::ExitCode;
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

pub async fn login_mode(url: &str, email: &str, password: &str) -> anyhow::Result<ExitCode> {
    let url = url.to_string();
    let url = Arc::new(url.to_string());
    info!("CLI running with url {}", url);
    let api_token = match login(
        &url,
        LoginForm {
            email: email.to_string(),
            password: password.to_string(),
        },
    )
    .await
    {
        Ok(api_token) => api_token,
        Err(_) => {
            setup_tracing(Uuid::default(), DeviceType::Cli);
            tracing::error!("Failed to login, did you register on {}/register ?", url);
            return Ok(ExitCode::FAILURE);
        }
    };
    setup_tracing(api_token, DeviceType::Cli);

    info!("Login successful");
    let email = Arc::new(email.to_string());
    let api_token = Arc::new(api_token.to_string());
    info!("CLI starting");
    let u = url.clone();
    let e = email.clone();
    let a = api_token.clone();
    let task_poller = tokio::spawn(async move {
        loop {
            let _ = task_poller(&u, e.as_ref(), a.as_ref()).await;
            tokio::time::sleep(Duration::from_secs(30)).await;
        }
    });
    let u = url.clone();
    let e = email.clone();
    let a = api_token.clone();
    let uptime_poller = tokio::spawn(async move {
        loop {
            let _ = report_uptime(&u, e.as_ref(), a.as_ref()).await;
            tokio::time::sleep(Duration::from_secs(30)).await;
        }
    });
    let u = url.clone();
    let e = email.clone();
    let a = api_token.clone();
    let bandwidth_poller = tokio::spawn(async move {
        loop {
            let _ = submit_bandwidth(&u, e.as_ref(), a.as_ref()).await;
            tokio::time::sleep(Duration::from_secs(30)).await;
        }
    });

    tokio::select! {
        o = task_poller => error!("task_poller failed {:?}", o),
        o = uptime_poller => error!("uptime_poller failed {:?}", o),
        o = bandwidth_poller => error!("bandwidth_poller failed {:?}", o)
    }
    Ok(ExitCode::SUCCESS)
}
