use block_mesh_common::points::raw_points;
use block_mesh_manager_database_domain::domain::apply_ref_bonus_for_daily_stat::apply_ref_bonus_for_dail_stat;
use block_mesh_manager_database_domain::domain::get_affiliate_tree_per_day::get_affiliate_tree_per_day;
use chrono::NaiveDate;
use database_utils::utils::instrument_wrapper::create_txn;
use sqlx::PgPool;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use uuid::Uuid;

pub async fn process_job(
    pool: PgPool,
    user_id: Uuid,
    daily_stat_id: Uuid,
    day: NaiveDate,
) -> anyhow::Result<()> {
    let mut transaction = create_txn(&pool).await?;
    let aff_tree = get_affiliate_tree_per_day(&mut transaction, &user_id, &day).await?;
    let mut sum = 0f64;
    aff_tree.iter().for_each(|i| {
        if i.level == 1 {
            sum += 0.2 * raw_points(i.uptime, i.tasks_count as i64);
        } else if i.level == 2 {
            sum += 0.1 * raw_points(i.uptime, i.tasks_count as i64);
        } else if i.level == 3 {
            sum += 0.05 * raw_points(i.uptime, i.tasks_count as i64);
        }
    });
    apply_ref_bonus_for_dail_stat(&mut transaction, &daily_stat_id, sum).await?;
    Ok(())
}

pub async fn ref_bonus_cron(
    queue: Arc<RwLock<HashSet<(Uuid, Uuid, NaiveDate)>>>,
    pool: PgPool,
) -> Result<(), anyhow::Error> {
    let zero_duration = Duration::from_millis(1_000);
    let non_zero_duration = Duration::from_millis(100);
    loop {
        if let Some(item) = queue.write().await.iter().next() {
            let poll_clone = pool.clone();
            let item_clone = *item;
            let handle = tokio::spawn(async move {
                let _ = process_job(poll_clone, item_clone.0, item_clone.1, item_clone.2).await;
            });
            let _ = handle.await;
            queue.write().await.remove(item);
        }
        let size = queue.read().await.len();
        if size == 0 {
            tokio::time::sleep(zero_duration).await;
        } else {
            tokio::time::sleep(non_zero_duration).await;
        }
    }
}
