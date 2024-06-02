use crate::database::daily_stat::get_daily_stats_by_user_id::get_daily_stats_by_user_id;
use crate::database::daily_stat::get_users_tank::{get_users_rank, RankedUsers};
use crate::database::invite_code::get_number_of_users_invited::get_number_of_users_invited;
use crate::database::invite_code::get_user_latest_invite_code::get_user_latest_invite_code;
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
    pub invite_code: String,
    pub number_of_users_invited: i64,
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
    let user_invite_code = get_user_latest_invite_code(&mut transaction, user.id)
        .await
        .map_err(Error::from)?;
    let number_of_users_invited = get_number_of_users_invited(&mut transaction, user.id)
        .await
        .map_err(Error::from)?;
    transaction.commit().await.map_err(Error::from)?;
    let template = DashboardTemplate {
        daily_stats,
        overall_task_count,
        rank: my_rank.rank.unwrap_or_default(),
        invite_code: user_invite_code.invite_code,
        number_of_users_invited,
    };
    Ok(template)
}
