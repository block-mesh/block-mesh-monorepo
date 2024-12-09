use block_mesh_common::interfaces::server_api::{ReferralSummary, TmpReferralSummary};
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

pub async fn get_user_referrals_summary(
    transaction: &mut Transaction<'_, Postgres>,
    user_id: &Uuid,
) -> anyhow::Result<ReferralSummary> {
    let summary: TmpReferralSummary = sqlx::query_as!(
        TmpReferralSummary,
        r#"
        SELECT
            count(*) as total_invites,
        	COUNT(*) FILTER (WHERE verified_email = TRUE) AS total_verified_email,
            COUNT(*) FILTER (WHERE proof_of_humanity = TRUE) AS total_verified_human,
            COUNT(*) FILTER (WHERE proof_of_humanity = TRUE AND verified_email = TRUE) AS total_eligible
        FROM users
        WHERE invited_by = $1
        "#,
        user_id
    )
    .fetch_one(&mut **transaction)
    .await?;
    Ok(ReferralSummary {
        total_invites: summary.total_invites.unwrap_or_default(),
        total_verified_email: summary.total_verified_email.unwrap_or_default(),
        total_verified_human: summary.total_verified_human.unwrap_or_default(),
        total_eligible: summary.total_eligible.unwrap_or_default(),
    })
}
