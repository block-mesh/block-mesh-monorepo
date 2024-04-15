use crate::database::daily_stat::get_daily_stats_by_user_id::get_daily_stats_by_user_id;
use crate::database::daily_stat::get_users_tank::{get_users_rank, RankedUsers};
use crate::domain::daily_stat::DailyStat;
use crate::errors::error::Error;
use crate::middlewares::authentication::Backend;
use askama::Template;
use askama_axum::IntoResponse;
use axum::Extension;
use axum_login::AuthSession;
use sqlx::PgPool;

#[derive(Template)]
#[template(path = "dashboard/dashboard.html")]
struct DashboardTemplate {
    pub overall_task_count: i64,
    pub rank: i64,
    pub daily_stats: Vec<DailyStat>,
}

#[tracing::instrument(name = "dashboard", skip(auth))]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Extension(auth): Extension<AuthSession<Backend>>,
) -> Result<impl IntoResponse, Error> {
    let mut transaction = pool.begin().await.map_err(Error::from)?;
    let user = auth.user.ok_or(Error::UserNotFound)?;
    let daily_stats = get_daily_stats_by_user_id(&mut transaction, &user.id)
        .await
        .map_err(Error::from)?;
    let overall_task_count = daily_stats.iter().map(|x| x.tasks_count).sum();
    let ranks = get_users_rank(&mut transaction).await?;
    let my_rank = ranks
        .into_iter()
        .find(|x| x.user_id == user.id)
        .unwrap_or(RankedUsers {
            user_id: user.id,
            total_tasks_count: Some(0),
            rank: Some(0),
        });
    transaction.commit().await.map_err(Error::from)?;
    let template = DashboardTemplate {
        daily_stats,
        overall_task_count,
        rank: my_rank.rank.unwrap_or_default(),
    };
    Ok(template)
}
