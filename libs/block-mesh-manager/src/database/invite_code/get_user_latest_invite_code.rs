use crate::domain::invite_code::InviteCode;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

pub async fn get_user_latest_invite_code(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
) -> anyhow::Result<InviteCode> {
    Ok(sqlx::query_as!(
        InviteCode,
        r#"SELECT
        id,
        invite_code,
        user_id,
        created_at
        FROM invite_codes WHERE user_id = $1 ORDER BY created_at DESC LIMIT 1"#,
        user_id
    )
    .fetch_one(&mut **transaction)
    .await?)
}
