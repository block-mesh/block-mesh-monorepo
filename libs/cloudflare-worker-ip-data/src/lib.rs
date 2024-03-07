use tracing_subscriber::fmt::format::Pretty;
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::prelude::*;
use tracing_web::{performance_layer, MakeConsoleWriter};
use worker::*;

static IP_HEADERS: [&str; 3] = ["cf-connecting-ip", "x-real-ip", "x-forwarded-for"];

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

#[event(fetch)]
async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    req.headers().entries().for_each(|(k, v)| {
        tracing::info!("{}: {}", k, v);
    });

    let ip: Vec<String> = IP_HEADERS
        .iter()
        .filter_map(|header| {
            let header_opt_res = req.headers().get(header);
            match header_opt_res {
                Ok(header) => header,
                Err(_) => return None,
            }
        })
        .collect();
    tracing::info!("IP Headers: {:?}", ip);
    // tracing::info!(request=?req, "Handling request");
    Response::ok("Hello, World!")
}
