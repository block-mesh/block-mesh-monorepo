use crate::database::api_token::get_api_token_by_user_id_and_status::get_api_token_by_usr_and_status;
use crate::database::daily_stat::get_daily_stats_by_user_id::get_daily_stats_by_user_id;
use crate::database::user::get_user_by_email::get_user_opt_by_email;
use crate::errors::error::Error;
use axum::{Extension, Json};
use block_mesh_common::interfaces::server_api::{GetStatsRequest, GetStatsResponse, Stat};
use block_mesh_manager_database_domain::domain::api_token::ApiTokenStatus;
use sqlx::PgPool;

#[tracing::instrument(name = "get_stats", skip_all)]
pub async fn handler(
    Extension(pool): Extension<PgPool>,
    Json(body): Json<GetStatsRequest>,
) -> Result<Json<GetStatsResponse>, Error> {
    let mut transaction = pool.begin().await.map_err(Error::from)?;
    let email = body.email.clone().to_ascii_lowercase();
    let user = get_user_opt_by_email(&mut transaction, &email)
        .await?
        .ok_or_else(|| Error::UserNotFound)?;
    let api_token =
        get_api_token_by_usr_and_status(&mut transaction, &user.id, ApiTokenStatus::Active)
            .await?
            .ok_or(Error::ApiTokenNotFound)?;
    if *api_token.token.as_ref() != body.api_token {
        return Err(Error::ApiTokenMismatch);
    }

    let daily_stats = get_daily_stats_by_user_id(&mut transaction, &user.id)
        .await
        .map_err(Error::from)?;
    transaction.commit().await.map_err(Error::from)?;
    Ok(Json(GetStatsResponse {
        stats: daily_stats
            .into_iter()
            .map(|i| Stat {
                day: i.day,
                tasks_count: i.tasks_count,
            })
            .collect(),
    }))
}
