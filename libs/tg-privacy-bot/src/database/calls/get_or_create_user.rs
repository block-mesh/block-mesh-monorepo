use crate::database::models::user::User;
use sqlx::{Postgres, Transaction};
use time::OffsetDateTime;
use uuid::Uuid;

pub async fn get_or_create_user(
    transaction: &mut Transaction<'_, Postgres>,
    tg_id: i64,
    username: &str,
) -> anyhow::Result<User> {
    let id = Uuid::new_v4();
    let now = OffsetDateTime::now_utc();
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users
        (id, tg_id, username, created_at)
        VALUES
        ($1, $2, $3, $4)
        ON CONFLICT (tg_id) DO UPDATE SET username = $3
        RETURNING id, tg_id, username, created_at
        "#,
        id,
        tg_id,
        username,
        now
    )
    .fetch_one(&mut **transaction)
    .await?;
    Ok(user)
}
