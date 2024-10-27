use sqlx::PgExecutor;

#[tracing::instrument(name = "health_check", skip_all)]
pub async fn health_check(executor: impl PgExecutor<'_>) -> sqlx::Result<()> {
    sqlx::query!(r#"SELECT current_database()"#)
        .fetch_one(executor)
        .await?;
    Ok(())
}
