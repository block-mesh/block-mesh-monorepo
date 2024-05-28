use chrono::Utc;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "Create Invite Code", skip(transaction), ret, err)]
pub(crate) async fn create_invite_code(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
    invite_code: String,
) -> anyhow::Result<Uuid> {
    let now = Utc::now();
    let id = Uuid::new_v4();
    sqlx::query!(
        r#"INSERT INTO invite_codes (id, created_at, invite_code, user_id) VALUES ($1, $2, $3, $4)"#,
        id,
        now,
        invite_code,
        user_id
    )
    .execute(&mut **transaction)
    .await?;
    Ok(id)
}
