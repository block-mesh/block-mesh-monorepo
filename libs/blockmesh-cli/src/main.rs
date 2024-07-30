use std::process::ExitCode;

use clap::Parser;

use block_mesh_common::cli::CliOpts;
use block_mesh_common::interfaces::server_api::LoginForm;

use crate::helpers::login;

mod helpers;

#[tokio::main]
pub async fn main() -> anyhow::Result<ExitCode> {
    let args = CliOpts::parse();

    let _api_token = login(LoginForm {
        email: args.email.clone(),
        password: args.password.clone(),
    })
    .await?;

    println!("Hello, world! {:#?}", args);

    Ok(ExitCode::SUCCESS)
}
