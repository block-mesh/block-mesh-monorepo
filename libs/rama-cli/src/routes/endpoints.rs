use super::data::{get_ja4h_info, get_tls_display_info_and_store};
use crate::rama_state::RamaState;
use block_mesh_common::interfaces::server_api::IdRequest;
use rama::http::{HeaderValue, Method, Response};
use rama::{
    Context,
    http::{BodyExtractExt, IntoResponse, Request, StatusCode, response::Json},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Default)]
pub struct Report {
    ja3: Option<String>,
    ja4: Option<String>,
    ja4h: Option<String>,
    ip: Option<String>,
}

pub async fn get_report(
    ctx: Context<Arc<RamaState>>,
    req: Request,
) -> Result<impl IntoResponse, Response> {
    let mut report = Report::default();
    let ja4h = get_ja4h_info(&req);
    if let Some(ja4h) = ja4h {
        report.ja4h = Some(ja4h.hash);
    }
    let (head, body) = req.into_parts();
    println!("method {}", head.method);
    if head.method == Method::POST {
        let x = body.try_into_json::<IdRequest>().await;
    }
    let ip = head
        .headers
        .get("cf-connecting-ip")
        .unwrap_or(&HeaderValue::from_static(""))
        .to_str()
        .unwrap_or_default()
        .to_string();
    report.ip = Some(ip);
    let tls_info = get_tls_display_info_and_store(&ctx)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response())?;
    if let Some(tls_info) = tls_info {
        report.ja3 = Some(tls_info.ja3.hash);
        report.ja4 = Some(tls_info.ja4.hash);
    }
    Ok(Json(report))
}
