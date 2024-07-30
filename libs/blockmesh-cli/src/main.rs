mod helpers;

use crate::helpers::login;
use block_mesh_common::cli::CliOpts;
use block_mesh_common::constants::BLOCK_MESH_APP_SERVER;
use block_mesh_common::interfaces::server_api::{GetTokenResponse, LoginForm};
use clap::Parser;
use std::process::ExitCode;

#[tokio::main]
pub async fn main() -> anyhow::Result<ExitCode> {
    let args = CliOpts::parse();

    let api_token = login(LoginForm {
        email: args.email.clone(),
        password: args.password.clone(),
    })
    .await?;

    println!("Hello, world! {:#?}", args);

    Ok(ExitCode::SUCCESS)
}
