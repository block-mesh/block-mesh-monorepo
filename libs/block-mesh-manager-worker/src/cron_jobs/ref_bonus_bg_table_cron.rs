use block_mesh_manager_database_domain::domain::daily_stat_background_job::DailyStatsBackgroundJob;
use block_mesh_manager_database_domain::domain::get_daily_stat_by_day::get_daily_stats_by_day;
use database_utils::utils::instrument_wrapper::{commit_txn, create_txn};
use sqlx::PgPool;
use std::env;
use std::time::Duration;

#[tracing::instrument(name = "pool", skip_all)]
pub async fn run_loop(pool: &PgPool) -> Result<(), anyhow::Error> {
    let mut transaction = create_txn(pool).await?;
    let job = match DailyStatsBackgroundJob::get_job(&mut transaction).await? {
        Some(job) => job,
        None => return Ok(()),
    };
    tracing::info!("job = {:#?}", job);
    let daily_stat = get_daily_stats_by_day(&mut transaction, &job.user_id, &job.day).await?;
    commit_txn(transaction).await?;
    DailyStatsBackgroundJob::process_job(
        pool.clone(),
        daily_stat.user_id,
        daily_stat.id,
        daily_stat.day,
    )
    .await?;
    let mut transaction = create_txn(pool).await?;
    DailyStatsBackgroundJob::delete_job(&mut transaction, &job.id).await?;
    commit_txn(transaction).await?;
    Ok(())
}

#[tracing::instrument(name = "ref_bonus_bg_table_cron", skip_all)]
pub async fn ref_bonus_bg_table_cron(pool: PgPool) -> Result<(), anyhow::Error> {
    let enable = env::var("REF_BONUS_BG_CRON_ENABLE")
        .unwrap_or("false".to_string())
        .parse()
        .unwrap_or(false);
    let ref_bonus_bg_table_cron_sleep = env::var("REF_BONUS_BG_TABLE_CRON")
        .unwrap_or("1000".to_string())
        .parse()
        .unwrap_or(1_000);
    let duration = Duration::from_millis(ref_bonus_bg_table_cron_sleep);
    loop {
        if enable {
            let _ = run_loop(&pool).await;
        }
        tokio::time::sleep(duration).await;
    }
}
