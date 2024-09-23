use crate::domain::task::TaskStatus;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[allow(clippy::too_many_arguments)]
pub async fn finish_task(
    transaction: &mut Transaction<'_, Postgres>,
    task_id: Uuid,
    response_code: Option<i32>,
    response_raw: Option<String>,
    status: TaskStatus,
    country: &str,
    ip: &str,
    asn: &str,
    colo: &str,
    response_time: f64,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE
        tasks
        SET
        response_code = $1,
        response_raw = $2,
        status = $3,
        country = $4,
        ip = $5,
        asn = $6,
        colo = $7,
        response_time = $8
        WHERE id = $9"#,
        response_code,
        response_raw,
        status.to_string(),
        country,
        ip,
        asn,
        colo,
        response_time,
        task_id
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
