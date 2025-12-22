use crate::domain::apply_ref_bonus_for_daily_stat::apply_ref_bonus_for_dail_stat;
use crate::domain::daily_stat::DailyStat;
use crate::domain::get_affiliate_tree_per_day::get_affiliate_tree_per_day;
use crate::domain::get_daily_stat_by_id::get_daily_stats_by_id;
use block_mesh_common::points::raw_points;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Postgres, Transaction};
use time::{Date, OffsetDateTime};
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct DailyStatsBackgroundJob {
    pub id: Uuid,
    pub user_id: Uuid,
    pub day: Date,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

impl DailyStatsBackgroundJob {
    #[tracing::instrument(name = "delete_job", skip_all, err)]
    pub async fn delete_job(
        transaction: &mut Transaction<'_, Postgres>,
        id: &Uuid,
    ) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM daily_stats_background_jobs WHERE id = $1
            "#,
            id
        )
        .execute(&mut **transaction)
        .await?;
        Ok(())
    }

    #[tracing::instrument(name = "get_job", skip_all, err)]
    pub async fn get_job(
        transaction: &mut Transaction<'_, Postgres>,
    ) -> anyhow::Result<Option<DailyStatsBackgroundJob>> {
        let job = sqlx::query_as!(
            DailyStatsBackgroundJob,
            r#"
            SELECT id, user_id, day, created_at, updated_at
            FROM daily_stats_background_jobs
            ORDER BY created_at ASC
            LIMIT 1
            "#
        )
        .fetch_optional(&mut **transaction)
        .await?;
        Ok(job)
    }

    #[tracing::instrument(name = "create_jobs", skip_all, err)]
    pub async fn create_jobs(
        transaction: &mut Transaction<'_, Postgres>,
        daily_stats: Vec<DailyStat>,
    ) -> anyhow::Result<()> {
        if daily_stats.is_empty() {
            return Ok(());
        }
        let values: Vec<String> = daily_stats
            .iter()
            .map(|i| format!("('{}'::uuid,  '{}')", i.user_id, i.day))
            .collect();
        let value_str = values.join(",");

        let query = format!(
            r#"
            INSERT INTO daily_stats_background_jobs (user_id, day)
            VALUES {}
            ON CONFLICT (day, user_id) DO NOTHING
        "#,
            value_str,
        );
        tracing::info!("query = {}", query);

        let _ = sqlx::query(&query)
            .execute(&mut **transaction)
            .await
            .map_err(|e| {
                tracing::error!(
                    "create_jobs failed to execute query size: {} , with error {:?}",
                    values.len(),
                    e
                );
                e
            })?;
        Ok(())
    }

    #[tracing::instrument(name = "process_job", skip_all)]
    pub async fn process_job(
        pool: PgPool,
        user_id: Uuid,
        daily_stat_id: Uuid,
        day: Date,
    ) -> anyhow::Result<()> {
        let mut transaction = create_txn(&pool).await?;
        let daily_stats = get_daily_stats_by_id(&mut transaction, &daily_stat_id).await?;
        if daily_stats.ref_bonus_applied {
            return Ok(());
        }
        let aff_tree = get_affiliate_tree_per_day(&mut transaction, &user_id, &day).await?;
        let mut sum = 0f64;
        aff_tree.iter().for_each(|i| {
            if i.level == 1 {
                sum += 0.2 * raw_points(i.uptime, i.tasks_count as i64, 0.0);
            } else if i.level == 2 {
                sum += 0.1 * raw_points(i.uptime, i.tasks_count as i64, 0.0);
            } else if i.level == 3 {
                sum += 0.05 * raw_points(i.uptime, i.tasks_count as i64, 0.0);
            }
        });
        apply_ref_bonus_for_dail_stat(&mut transaction, &daily_stat_id, sum).await?;
        commit_txn(transaction).await?;
        Ok(())
    }
}
