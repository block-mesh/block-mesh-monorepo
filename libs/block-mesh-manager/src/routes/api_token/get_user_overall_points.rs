use crate::database::daily_stat::get_daily_stats_by_user_id::get_daily_stats_by_user_id;
use crate::database::perks::get_user_perks::get_user_perks;
use crate::database::user::get_user_by_email::get_user_opt_by_email;
use crate::errors::error::Error;
use crate::startup::application::AppState;
use crate::utils::points::{calc_one_time_bonus_points, calc_points_daily, calc_total_points};
use anyhow::anyhow;
use axum::extract::State;
use axum::Json;
use block_mesh_common::interfaces::server_api::{
    GetUserOverallPointsRequest, GetUserOverallPointsResponse,
};
use block_mesh_manager_database_domain::domain::aggregate::AggregateName::{Tasks, Uptime};
use block_mesh_manager_database_domain::domain::bulk_get_or_create_aggregate_by_user_and_name::bulk_get_or_create_aggregate_by_user_and_name;
use block_mesh_manager_database_domain::domain::create_daily_stat::get_or_create_daily_stat;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use std::cmp::max;
use std::sync::Arc;

#[tracing::instrument(name = "get_user_overall_points", skip_all)]
pub async fn handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<GetUserOverallPointsRequest>,
) -> Result<Json<GetUserOverallPointsResponse>, Error> {
    if body.api_key.is_empty() || body.api_key.as_str() != state.team_api_key.as_str() {
        return Err(Error::Unauthorized);
    }

    let email = body.email.trim().to_ascii_lowercase();
    if email.is_empty() {
        return Err(Error::BadRequest("email is required".to_string()));
    }

    let mut read_transaction = create_txn(&state.dashboard_pool).await?;
    let user = get_user_opt_by_email(&mut read_transaction, &email)
        .await?
        .ok_or(Error::UserNotFound)?;
    let perks = get_user_perks(&mut read_transaction, &user.id).await?;
    let daily_stats = get_daily_stats_by_user_id(&mut read_transaction, &user.id).await?;
    commit_txn(read_transaction).await?;

    let daily_points_sum = daily_stats
        .iter()
        .map(|stat| calc_points_daily(stat.uptime, stat.tasks_count, stat.ref_bonus, &perks))
        .sum::<f64>();

    let mut write_transaction = create_txn(&state.pool).await?;
    let _ = get_or_create_daily_stat(&mut write_transaction, &user.id, None).await?;
    let aggregates =
        bulk_get_or_create_aggregate_by_user_and_name(&mut write_transaction, &user.id).await?;
    let uptime = aggregates
        .iter()
        .find(|aggregate| aggregate.name == Uptime)
        .ok_or_else(|| anyhow!("Uptime not found"))?;
    let tasks = aggregates
        .iter()
        .find(|aggregate| aggregate.name == Tasks)
        .ok_or_else(|| anyhow!("Tasks not found"))?;
    commit_txn(write_transaction).await?;

    let overall_uptime = max(
        uptime.value.as_f64().unwrap_or_default() as u64,
        daily_stats.iter().map(|stat| stat.uptime).sum::<f64>() as u64,
    );
    let overall_task_count = max(
        tasks.value.as_i64().unwrap_or_default(),
        daily_stats.iter().map(|stat| stat.tasks_count).sum::<i64>(),
    );
    let one_time_bonus_points =
        calc_one_time_bonus_points(overall_uptime as f64, overall_task_count, 0.0, &perks) as u64;
    let overall_points = max(
        calc_total_points(overall_uptime as f64, overall_task_count, 0.0, &perks) as u64,
        one_time_bonus_points + daily_points_sum as u64,
    ) as f64;

    Ok(Json(GetUserOverallPointsResponse {
        email,
        overall_points,
    }))
}
