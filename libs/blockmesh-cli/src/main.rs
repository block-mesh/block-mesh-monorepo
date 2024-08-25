use std::process::ExitCode;

use block_mesh_common::cli::{CliOptMod, CliOpts};
use block_mesh_common::constants::DeviceType;
use block_mesh_common::interfaces::server_api::{DashboardRequest, LoginForm, RegisterForm};
use blockmesh_cli::helpers::{dashboard, login_to_network, register};
use blockmesh_cli::login_mode::login_mode;
use clap::Parser;
use logger_general::tracing::setup_tracing;
use uuid::Uuid;

#[macro_use]
pub extern crate tracing;

mod helpers;

#[tokio::main]
pub async fn main() -> anyhow::Result<ExitCode> {
    let args = CliOpts::parse();
    match args.mode {
        CliOptMod::Login => {
            login_mode(
                &args.url.clone(),
                &args.email,
                &args.password,
                args.depin_aggregator,
            )
            .await?;
        }
        CliOptMod::Register => {
            setup_tracing(Uuid::default(), DeviceType::Cli);
            register(
                &args.url,
                &RegisterForm {
                    email: args.email,
                    password: args.password.clone(),
                    password_confirm: args.password,
                    invite_code: args.invite_code,
                },
            )
            .await?;
        }
        CliOptMod::Dashboard => {
            setup_tracing(Uuid::default(), DeviceType::Cli);
            let api_token = login_to_network(
                &args.url,
                LoginForm {
                    email: args.email.clone(),
                    password: args.password.clone(),
                },
            )
            .await?;
            dashboard(
                &args.url,
                &DashboardRequest {
                    email: args.email.clone(),
                    api_token,
                },
            )
            .await?;
        }
    }
    Ok(ExitCode::SUCCESS)
}
