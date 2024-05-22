use tracing_subscriber::fmt::format::Pretty;
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::prelude::*;
use tracing_web::{performance_layer, MakeConsoleWriter};
use worker::*;

pub const LATEST_RELEASE: &str =
    "https://api.github.com/repos/block-mesh/block-mesh-monorepo/releases/latest";

#[event(start)]
fn start() {
    let fmt_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_ansi(false) // Only partially supported across JavaScript runtimes
        .with_timer(UtcTime::rfc_3339()) // std::time is not available in browsers
        .with_writer(MakeConsoleWriter); // write events to the console
    let perf_layer = performance_layer().with_details_from_fields(Pretty::default());
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(perf_layer)
        .init();
}

pub async fn get_release() -> anyhow::Result<()> {
    let latest_response: serde_json::Value = reqwest::Client::new()
        .get(LATEST_RELEASE)
        .send()
        .await?
        .json()
        .await?;
    let latest_assets = latest_response
        .get("assets")
        .ok_or(anyhow::anyhow!("No assets found"))?;
    let _latest_asset = latest_assets
        .get(0)
        .ok_or(anyhow::anyhow!("Latest asset not assets found"))?;

    Ok(())
}

#[event(fetch)]
async fn main(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    //         "https://releases.myapp.com/{{target}}/{{arch}}/{{current_version}}"
    let url = req.url()?;
    let path = url.path();
    let parts: Vec<&str> = path.split('/').collect();
    if parts.len() != 4 {
        return Response::error("Invalid path", 400);
    }
    /*
        {
      "version": "0.2.0",
      "pub_date": "2020-09-18T12:29:53+01:00",
      "url": "https://mycompany.example.com/myapp/releases/myrelease.tar.gz",
      "signature": "Content of the relevant .sig file",
      "notes": "These are some release notes"
    }
         */

    let (_, _target, _arch, _current_version) = (parts[0], parts[1], parts[2], parts[3]);
    Response::ok("")
}
