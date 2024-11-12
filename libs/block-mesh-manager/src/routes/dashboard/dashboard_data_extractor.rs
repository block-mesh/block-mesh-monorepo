use chrono::Utc;
use num_traits::abs;
use sqlx::PgPool;
use std::cmp::max;
use std::sync::Arc;
#[allow(unused_imports)]
use tracing::Level;
use uuid::Uuid;

use block_mesh_common::interfaces::server_api::{
    CallToActionUI, DailyStatForDashboard, DashboardResponse, PerkUI, Referral,
};

use crate::database::call_to_action::get_user_calls_to_action::get_user_call_to_action;
use crate::database::daily_stat::get_daily_stats_by_user_id::get_daily_stats_by_user_id;
use crate::database::invite_code::get_number_of_users_invited::get_number_of_users_invited;
use crate::database::invite_code::get_user_latest_invite_code::get_user_latest_invite_code;
use crate::database::invite_code::get_user_referrals::get_user_referrals;
use crate::database::perks::get_user_perks::get_user_perks;
use crate::database::users_ip::get_user_ips::get_user_ips;
use crate::errors::error::Error;
use crate::startup::application::AppState;
use crate::utils::cache_envar::get_envar;
use crate::utils::points::{calc_one_time_bonus_points, calc_points_daily, calc_total_points};
use block_mesh_common::feature_flag_client::{get_flag_value_from_map, FlagValue};
use block_mesh_manager_database_domain::domain::aggregate::AggregateName;
use block_mesh_manager_database_domain::domain::get_or_create_aggregate_by_user_and_name::get_or_create_aggregate_by_user_and_name;
use block_mesh_manager_database_domain::domain::get_user_opt_by_id::get_user_opt_by_id;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use regex::Regex;

#[tracing::instrument(name = "dashboard_data_extractor", skip_all)]
pub async fn dashboard_data_extractor(
    pool: &PgPool,
    user_id: Uuid,
    state: Arc<AppState>,
) -> anyhow::Result<DashboardResponse> {
    let mut transaction = create_txn(pool).await?;
    let user = get_user_opt_by_id(&mut transaction, &user_id)
        .await?
        .ok_or_else(|| Error::UserNotFound)?;
    let tasks =
        get_or_create_aggregate_by_user_and_name(&mut transaction, AggregateName::Tasks, &user_id)
            .await?;
    let number_of_users_invited = get_number_of_users_invited(&mut transaction, user_id)
        .await
        .map_err(Error::from)?;
    let uptime_aggregate =
        get_or_create_aggregate_by_user_and_name(&mut transaction, AggregateName::Uptime, &user_id)
            .await
            .map_err(Error::from)?;
    let referrals = get_user_referrals(&mut transaction, user_id)
        .await
        .map_err(Error::from)?;

    let user_invite_code = get_user_latest_invite_code(&mut transaction, user_id)
        .await
        .map_err(Error::from)?;

    let interval = get_flag_value_from_map(
        &state.flags,
        "polling_interval",
        FlagValue::Number(120_000.0),
    );
    let interval: f64 =
        <FlagValue as TryInto<f64>>::try_into(interval.to_owned()).unwrap_or_default();

    let now = Utc::now();
    let diff = now - uptime_aggregate.updated_at;
    let sec_diff = abs(diff.num_seconds());
    let limit = 5;
    let user_ips = get_user_ips(&mut transaction, &user_id, limit).await?;
    let connected_buffer = get_envar("CONNECTED_BUFFER").await.parse().unwrap_or(10);
    let connected =
        sec_diff < connected_buffer * ((interval * 2.0) as i64).checked_div(1_000).unwrap_or(240);
    let calls_to_action = get_user_call_to_action(&mut transaction, user_id).await?;
    let perks = get_user_perks(&mut transaction, user_id).await?;
    let daily_stats: Vec<DailyStatForDashboard> =
        get_daily_stats_by_user_id(&mut transaction, &user_id)
            .await?
            .into_iter()
            .map(|i| {
                let points = calc_points_daily(i.uptime, i.tasks_count, &perks);
                DailyStatForDashboard {
                    tasks_count: i.tasks_count,
                    uptime: i.uptime,
                    points,
                    day: i.day,
                }
            })
            .collect();
    let overall_uptime = max(
        uptime_aggregate.value.as_f64().unwrap_or_default() as u64,
        daily_stats.iter().map(|i| i.uptime).sum::<f64>() as u64,
    );
    let overall_task_count = max(
        tasks.value.as_i64().unwrap_or_default(),
        daily_stats.iter().map(|i| i.tasks_count).sum::<i64>(),
    );
    let one_time_bonus_points =
        calc_one_time_bonus_points(overall_uptime as f64, overall_task_count, &perks) as u64;
    let points = max(
        calc_total_points(overall_uptime as f64, overall_task_count, &perks) as u64,
        one_time_bonus_points + daily_stats.iter().map(|i| i.points).sum::<f64>() as u64,
    ) as f64;
    let download = get_or_create_aggregate_by_user_and_name(
        &mut transaction,
        AggregateName::Download,
        &user_id,
    )
    .await?;

    let upload =
        get_or_create_aggregate_by_user_and_name(&mut transaction, AggregateName::Upload, &user_id)
            .await?;

    let latency = get_or_create_aggregate_by_user_and_name(
        &mut transaction,
        AggregateName::Latency,
        &user_id,
    )
    .await?;

    commit_txn(transaction).await?;
    Ok(DashboardResponse {
        wallet_address: user.wallet_address,
        user_ips,
        calls_to_action: calls_to_action
            .into_iter()
            .map(|i| CallToActionUI {
                id: i.id,
                name: i.name.to_string(),
                status: i.status,
            })
            .collect(),
        verified_email: user.verified_email,
        referrals: referrals
            .into_iter()
            .map(|i| Referral {
                created_at: i.created_at,
                verified_email: i.verified_email,
                email: {
                    let s: Vec<&str> = i.email.split('@').collect();
                    let re = Regex::new(r"[A-Za-z]").unwrap();
                    let result = re.replace_all(s[0], "x");
                    format!("{}@{}", result, s[1])
                },
            })
            .collect(),
        upload: upload.value.as_f64().unwrap_or_default(),
        download: download.value.as_f64().unwrap_or_default(),
        latency: latency.value.as_f64().unwrap_or_default(),
        points,
        uptime: overall_uptime as f64,
        tasks: overall_task_count,
        number_of_users_invited,
        invite_code: user_invite_code.invite_code,
        connected,
        daily_stats: daily_stats.iter().take(10).rev().cloned().collect(),
        perks: perks
            .into_iter()
            .map(|i| PerkUI {
                id: i.id,
                name: i.name.to_string(),
                multiplier: i.multiplier,
                one_time_bonus: i.one_time_bonus,
            })
            .collect(),
    })
}
