use crate::domain::daily_stat::{DailyStat, DailyStatStatus, DailyStatTmp};
use chrono::{NaiveDate, Utc};
use sqlx::{Postgres, Transaction};
use uuid::Uuid;
#[tracing::instrument(name = "get_or_create_daily_stat", skip_all)]
pub async fn get_or_create_daily_stat(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
    input_day: Option<NaiveDate>,
) -> anyhow::Result<DailyStat> {
    let now = Utc::now();
    let day = match input_day {
        Some(d) => d,
        None => now.date_naive(),
    };
    let id = Uuid::new_v4();
    let daily_stat = sqlx::query_as!(
        DailyStatTmp,
        r#"
           WITH
            extant AS (
                SELECT id, created_at, user_id, tasks_count, status, day, uptime, updated_at, ref_bonus, ref_bonus_applied FROM daily_stats
                WHERE user_id = $3
                AND day = $6
                AND status = $5
            ),
            extant2 AS (
                SELECT id, created_at, user_id, tasks_count, status, day, uptime, updated_at, ref_bonus, ref_bonus_applied FROM daily_stats
                WHERE user_id = $3
                AND day = $6
                AND status = $11
            ),
            inserted AS (
                INSERT INTO daily_stats (id, created_at, user_id, tasks_count, status, day, uptime, updated_at, ref_bonus, ref_bonus_applied)
                SELECT $1, $2, $3, $4, $5, $6, $7, $8, $9 , $10
                WHERE NOT EXISTS (SELECT FROM extant)
                RETURNING id, created_at, user_id, tasks_count, status, day, uptime, updated_at, ref_bonus, ref_bonus_applied
            )
        SELECT id, created_at, user_id, tasks_count, status, day, uptime, updated_at, ref_bonus, ref_bonus_applied FROM inserted
        UNION ALL
        SELECT id, created_at, user_id, tasks_count, status, day, uptime, updated_at, ref_bonus, ref_bonus_applied FROM extant
        UNION ALL
        SELECT id, created_at, user_id, tasks_count, status, day, uptime, updated_at, ref_bonus, ref_bonus_applied FROM extant2
        "#,
        id,
        now.clone(),
        user_id,
        0,
        DailyStatStatus::OnGoing.to_string(),
        day,
        0.0,
        now,
        0.0,
        false,
        DailyStatStatus::Finalized.to_string()
    )
    .fetch_one(&mut **transaction)
    .await?;
    let daily_stat = DailyStat {
        id: daily_stat.id.expect("MISSING ID"),
        user_id: daily_stat.user_id.expect("MISSING USEr ID"),
        tasks_count: daily_stat.tasks_count.expect("MISSING Tasks Count").into(),
        uptime: daily_stat.uptime.expect("MISSING Uptime"),
        status: daily_stat.status.expect("MISSING Status").into(),
        day: daily_stat.day.expect("MISSING Day"),
        created_at: daily_stat.created_at.expect("MISSING Time Created"),
        updated_at: daily_stat.updated_at.expect("MISSING Time Updated"),
        ref_bonus: daily_stat.ref_bonus.expect("MISSING ref_bonus"),
        ref_bonus_applied: daily_stat
            .ref_bonus_applied
            .expect("MISSING ref_bonus_applied"),
    };
    Ok(daily_stat)
}
