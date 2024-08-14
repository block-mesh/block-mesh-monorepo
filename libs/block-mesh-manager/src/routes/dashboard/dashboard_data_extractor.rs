use sqlx::PgPool;
#[allow(unused_imports)]
use tracing::Level;
use uuid::Uuid;

use block_mesh_common::interfaces::server_api::{
    CallToActionUI, DailyStatForDashboard, DashboardResponse, PerkUI, Referral,
};

use crate::database::aggregate::get_or_create_aggregate_by_user_and_name_no_transaction::get_or_create_aggregate_by_user_and_name_no_transaction;
use crate::database::call_to_action::get_user_calls_to_action::get_user_call_to_action;
use crate::database::daily_stat::get_daily_stats_by_user_id::get_daily_stats_by_user_id;
use crate::database::invite_code::get_number_of_users_invited::get_number_of_users_invited;
use crate::database::invite_code::get_user_latest_invite_code::get_user_latest_invite_code;
use crate::database::invite_code::get_user_referrals::get_user_referrals;
use crate::database::perks::get_user_perks::get_user_perks;
use crate::database::uptime_report::get_user_uptimes::get_user_uptimes;
use crate::database::user::get_user_by_id::get_user_opt_by_id;
use crate::domain::aggregate::AggregateName;
use crate::errors::error::Error;
use crate::utils::points::{calc_points_daily, calc_total_points};
use regex::Regex;

pub async fn dashboard_data_extractor(
    pool: &PgPool,
    user_id: Uuid,
) -> anyhow::Result<DashboardResponse> {
    let mut transaction = pool.begin().await.map_err(Error::from)?;
    let user = get_user_opt_by_id(&mut transaction, &user_id)
        .await?
        .ok_or_else(|| Error::UserNotFound)?;
    let tasks = get_or_create_aggregate_by_user_and_name_no_transaction(
        &mut transaction,
        AggregateName::Tasks,
        user_id,
    )
    .await?;
    let overall_task_count = tasks.value.as_i64().unwrap_or_default();
    let number_of_users_invited = get_number_of_users_invited(&mut transaction, user_id)
        .await
        .map_err(Error::from)?;
    let uptime_aggregate = get_or_create_aggregate_by_user_and_name_no_transaction(
        &mut transaction,
        AggregateName::Uptime,
        user_id,
    )
    .await
    .map_err(Error::from)?;
    let referrals = get_user_referrals(&mut transaction, user_id)
        .await
        .map_err(Error::from)?;
    let overall_uptime = uptime_aggregate.value.as_f64().unwrap_or_default();
    let user_invite_code = get_user_latest_invite_code(&mut transaction, user_id)
        .await
        .map_err(Error::from)?;

    let uptimes = get_user_uptimes(&mut transaction, user_id, 2).await?;
    let connected = if uptimes.len() == 2 {
        let diff = uptimes[0].created_at - uptimes[1].created_at;
        if diff.num_seconds() < 60 {
            true
        } else {
            false
        }
    } else {
        false
    };
    let calls_to_action = get_user_call_to_action(&mut transaction, user_id).await?;
    let perks = get_user_perks(&mut transaction, user_id).await?;
    let daily_stats = get_daily_stats_by_user_id(&mut transaction, &user_id)
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
        .rev()
        .collect();
    let points = calc_total_points(overall_uptime, overall_task_count, &perks);
    let download = get_or_create_aggregate_by_user_and_name_no_transaction(
        &mut transaction,
        AggregateName::Download,
        user_id,
    )
    .await?;

    let upload = get_or_create_aggregate_by_user_and_name_no_transaction(
        &mut transaction,
        AggregateName::Upload,
        user_id,
    )
    .await?;

    let latency = get_or_create_aggregate_by_user_and_name_no_transaction(
        &mut transaction,
        AggregateName::Latency,
        user_id,
    )
    .await?;

    transaction.commit().await.map_err(Error::from)?;
    Ok(DashboardResponse {
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
                    let s: Vec<&str> = i.email.split("@").collect();
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
        uptime: overall_uptime,
        tasks: overall_task_count,
        number_of_users_invited,
        invite_code: user_invite_code.invite_code,
        connected,
        daily_stats,
        perks: perks
            .into_iter()
            .map(|i| PerkUI {
                id: i.id,
                name: i.name.to_string(),
                multiplier: i.multiplier,
            })
            .collect(),
    })
}
