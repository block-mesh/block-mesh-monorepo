use crate::database::aggregate::get_or_create_aggregate_by_user_and_name_no_transaction::get_or_create_aggregate_by_user_and_name_no_transaction;
use crate::database::api_token::find_token::find_token;
use crate::database::daily_stat::create_daily_stat::create_daily_stat;
use crate::database::daily_stat::get_daily_stat_by_user_id_and_day::get_daily_stat_by_user_id_and_day;
use crate::database::user::get_user_by_id::get_user_opt_by_id;
use crate::domain::aggregate::AggregateName;
use crate::errors::error::Error;
use crate::startup::application::AppState;
use crate::worker::db_agg::{Table, UpdateBulkMessage};
use crate::worker::db_cleaner_cron::EnrichIp;
use axum::extract::{ConnectInfo, Query, State};
use axum::{Extension, Json};
use block_mesh_common::feature_flag_client::FlagValue;
use block_mesh_common::interfaces::server_api::{ReportUptimeRequest, ReportUptimeResponse};
use chrono::Utc;
use http::{HeaderMap, HeaderValue, StatusCode};
use sqlx::PgPool;
use std::net::SocketAddr;
use std::sync::Arc;

pub fn resolve_ip(
    query_ip: &Option<String>,
    header_ip: &Option<&HeaderValue>,
    addr_ip: String,
) -> String {
    if header_ip.is_some() {
        header_ip.unwrap().to_str().unwrap_or_default().to_string()
    } else if query_ip.is_some() {
        query_ip.clone().unwrap().clone()
    } else {
        addr_ip
    }
}

#[tracing::instrument(name = "report_uptime", level = "trace", skip(pool, query, state), ret)]
pub async fn handler(
    headers: HeaderMap,
    Extension(pool): Extension<PgPool>,
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Query(query): Query<ReportUptimeRequest>,
) -> Result<Json<ReportUptimeResponse>, Error> {
    let mut transaction = pool.begin().await.map_err(Error::from)?;
    let api_token = find_token(&mut transaction, &query.api_token)
        .await?
        .ok_or(Error::ApiTokenNotFound)?;
    let user = get_user_opt_by_id(&mut transaction, &api_token.user_id)
        .await?
        .ok_or_else(|| Error::UserNotFound)?;
    if let Some(metadata) = query.metadata {
        println!("Inserting analytics");
        tracing::trace!("Inserting analytics");
        sqlx::query!(r#"
        INSERT INTO analytics (user_id, depin_aggregator, device_type, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $4)
        ON CONFLICT (user_id, depin_aggregator) DO UPDATE SET updated_at = $4
        "#, user.id, metadata.depin_aggregator, metadata.device_type.to_string(), Utc::now()).execute(&pool).await?;
    }
    if user.email.to_ascii_lowercase() != query.email.to_ascii_lowercase() {
        return Err(Error::UserNotFound);
    }
    let header_ip = headers.get("cf-connecting-ip");
    let ip = resolve_ip(&query.ip, &header_ip, addr.ip().to_string());
    let daily_stat_opt =
        get_daily_stat_by_user_id_and_day(&mut transaction, user.id, Utc::now().date_naive())
            .await?;
    if daily_stat_opt.is_none() {
        create_daily_stat(&mut transaction, user.id).await?;
    }


    let interval = state
        .flags
        .get("polling_interval")
        .unwrap_or(&FlagValue::Number(120_000.0));
    let interval: f64 =
        <FlagValue as TryInto<f64>>::try_into(interval.to_owned()).unwrap_or_default();

    let aggregate = get_or_create_aggregate_by_user_and_name_no_transaction(
        &mut transaction,
        AggregateName::Uptime,
        user.id,
    )
    .await
    .map_err(Error::from)?;
    transaction.commit().await.map_err(Error::from)?;

    let now = Utc::now();
    let diff = now - aggregate.updated_at.unwrap_or(now);

    if diff.num_seconds() < ((interval * 2.0) as i64).checked_div(1_000).unwrap_or(240) {
        if daily_stat_opt.is_some() {
            let _ = state
                .tx_sql_agg
                .send(UpdateBulkMessage {
                    id: daily_stat_opt.unwrap().id,
                    value: serde_json::Value::from(diff.num_seconds() as f64),
                    table: Table::DailyStat,
                })
                .await;
        }
        let sum = aggregate.value.as_f64().unwrap_or_default() + diff.num_seconds() as f64;
        let _ = state
            .tx_sql_agg
            .send(UpdateBulkMessage {
                id: aggregate.id.0.unwrap_or_default(),
                value: serde_json::Value::from(sum),
                table: Table::Aggregate,
            })
            .await;
    } else {
        let _ = state
            .tx_sql_agg
            .send(UpdateBulkMessage {
                id: aggregate.id.0.unwrap_or_default(),
                value: serde_json::Value::from(aggregate.value.as_f64().unwrap_or_default()),
                table: Table::Aggregate,
            })
            .await;
    }

    let flag = state
        .flags
        .get("send_cleanup_to_rayon")
        .unwrap_or(&FlagValue::Boolean(false));
    let flag: bool = <FlagValue as TryInto<bool>>::try_into(flag.to_owned()).unwrap_or_default();
    if flag {
        let _ = state.cleaner_tx.send(EnrichIp {
            user_id: user.id,
            ip: ip.clone(),
        });
    }

    Ok(Json(ReportUptimeResponse {
        status_code: u16::from(StatusCode::OK),
    }))
}
