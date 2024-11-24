use crate::domain::call_to_action::CallToAction;
use sqlx::{query_as, Postgres, Transaction};
use uuid::Uuid;

#[allow(dead_code)]
struct Id {
    id: Uuid,
}

pub async fn get_user_call_to_action(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
) -> anyhow::Result<Vec<CallToAction>> {
    let calls_to_action = query_as!(
        CallToAction,
        r#"
        SELECT
        id, user_id, name, created_at, status
        FROM call_to_actions
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_all(&mut **transaction)
    .await?;
    Ok(calls_to_action)
}
