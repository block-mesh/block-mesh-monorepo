use crate::domain::spam_email::SpamEmail;
use sqlx::{Postgres, Transaction};

pub async fn get_spam_emails(
    transaction: &mut Transaction<'_, Postgres>,
) -> anyhow::Result<Vec<SpamEmail>> {
    let spam_emails = sqlx::query_as!(
        SpamEmail,
        r#"
        SELECT
        id,
        domain,
        created_at,
        updated_at
        FROM spam_emails
        "#,
    )
    .fetch_all(&mut **transaction)
    .await?;
    Ok(spam_emails)
}
