use crate::domain::invite_code::InviteCode;
use sqlx::{Postgres, Transaction};

pub async fn get_user_opt_by_invited_code(
    transaction: &mut Transaction<'_, Postgres>,
    invite_code: String,
) -> anyhow::Result<Option<InviteCode>> {
    Ok(sqlx::query_as!(
        InviteCode,
        r#"SELECT
        id,
        invite_code,
        user_id,
        created_at
        FROM invite_codes WHERE invite_code = $1 LIMIT 1"#,
        invite_code
    )
    .fetch_optional(&mut **transaction)
    .await?)
}
