use super::data::{get_ja4h_info, get_tls_display_info_and_store};
use crate::db::get_or_create_rama_id;
use crate::rama_state::RamaState;
use block_mesh_common::interfaces::server_api::IdRequest;
use database_utils::utils::health_check::health_check;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use rama::http::{Body, HeaderValue, Method, Response};
use rama::{
    Context,
    http::{
        BodyExtractExt,
        IntoResponse,
        Request,
        StatusCode,
        //response::Json
    },
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub fn cors_response() -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .header("Access-Control-Allow-Origin", "*") // Allow all origins (adjust as needed)
        .header("Access-Control-Allow-Methods", "*") // Allowed methods
        .header("Access-Control-Allow-Headers", "*") // Allowed headers
        .body(Body::empty())
        .unwrap()
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Report {
    ja3: Option<String>,
    ja4: Option<String>,
    ja4h: Option<String>,
    ip: Option<String>,
}

pub async fn server_health() -> Result<impl IntoResponse, Response> {
    Ok(cors_response())
}
pub async fn version() -> impl IntoResponse {
    (StatusCode::OK, env!("CARGO_PKG_VERSION"))
}

pub async fn db_health(ctx: Context<Arc<RamaState>>) -> Result<impl IntoResponse, Response> {
    let state = ctx.state().clone();
    if let Ok(mut transaction) = create_txn(&state.db_pool).await {
        if health_check(&mut *transaction).await.is_ok() && commit_txn(transaction).await.is_ok() {
            return Ok((StatusCode::OK, "OK"));
        }
    }
    Ok((StatusCode::INTERNAL_SERVER_ERROR, "ERROR"))
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
    let state = ctx.state().clone();
    let ip = if state.environment.as_str() != "local" {
        head.headers
            .get("cf-connecting-ip")
            .unwrap_or(&HeaderValue::from_static(""))
            .to_str()
            .unwrap_or_default()
            .to_string()
    } else {
        "127.0.0.1".to_string()
    };
    report.ip = Some(ip);
    let tls_info = get_tls_display_info_and_store(&ctx)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response())?;
    if let Some(tls_info) = tls_info {
        report.ja3 = Some(tls_info.ja3.hash);
        report.ja4 = Some(tls_info.ja4.hash);
    }
    if head.method == Method::POST {
        let report = report.clone();
        let body = body.try_into_json::<IdRequest>().await;
        let state = ctx.state().clone();
        if let Ok(mut transaction) = create_txn(&state.db_pool).await {
            if let Ok(body) = body {
                let _ = get_or_create_rama_id(
                    &mut transaction,
                    &body.email,
                    &body.api_token,
                    &report.ja3.unwrap_or_default(),
                    &report.ja4.unwrap_or_default(),
                    &report.ja4h.unwrap_or_default(),
                    &report.ip.unwrap_or_default(),
                )
                .await;
            }
            let _ = commit_txn(transaction).await;
        }
    }
    Ok(cors_response())
    // Ok(Json(report))
}
