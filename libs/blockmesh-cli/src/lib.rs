mod helpers;

use crate::helpers::{login, report_uptime, submit_bandwidth, task_poller};
use block_mesh_common::constants::DeviceType;
use block_mesh_common::interfaces::server_api::LoginForm;
use logger_general::tracing::setup_tracing;
use std::ffi::CStr;
use std::io::Write;
use std::os::raw::c_char;
use std::process::ExitCode;
use std::sync::Arc;
use std::time::Duration;
use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::runtime::Builder;
use uuid::Uuid;

/// # Safety
/// This method should be called by any external program that want to use BlockMesh Network CLI
// #[allow(unsafe_code)]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn run_lib(email: *const c_char, password: *const c_char) -> i8 {
    let email = match unsafe { CStr::from_ptr(email) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to load email {}", e);
            return -1;
        }
    };
    let password = match unsafe { CStr::from_ptr(password) }.to_str() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to load password {}", e);
            return -1;
        }
    };

    let runtime = Arc::new(
        Builder::new_multi_thread()
            .thread_name("blockmesh-cli")
            .enable_all()
            .build()
            .unwrap(),
    );

    runtime.block_on(async {
        println!("IN BLOCKING 48");
        let _ = io::stdout().flush().await;
        let _ = std::io::stdout().flush();
        let _ = run_cli(email, password).await;
    });
    -1
}

pub async fn run_cli(email: &str, password: &str) -> anyhow::Result<ExitCode> {
    println!("RUN CLI");
    let api_token = match login(LoginForm {
        email: email.to_string(),
        password: password.to_string(),
    })
    .await
    {
        Ok(api_token) => api_token,
        Err(_) => {
            setup_tracing(Uuid::default(), DeviceType::Cli);
            tracing::error!(
                "Failed to login, did you register on https://app.blockmesh.xyz/register ?"
            );
            return Ok(ExitCode::FAILURE);
        }
    };
    setup_tracing(api_token, DeviceType::Cli);

    tracing::info!("Login successful");
    let email = Arc::new(email.to_string());
    let api_token = Arc::new(api_token.to_string());
    tracing::info!("CLI starting");
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
    }
    Ok(ExitCode::SUCCESS)
}
