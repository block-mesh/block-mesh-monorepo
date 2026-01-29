use std::future::Future;
use std::pin::Pin;

pub fn stale_txn_guard(
    pool_name: &'static str,
) -> impl for<'c> Fn(
    &'c mut sqlx::postgres::PgConnection,
    sqlx::pool::PoolConnectionMetadata,
) -> Pin<Box<dyn Future<Output = Result<bool, sqlx::Error>> + Send + 'c>>
       + Send
       + Sync
       + 'static {
    move |conn, _meta| {
        Box::pin(async move {
            match sqlx::query_scalar::<_, Option<i64>>("SELECT txid_current_if_assigned()::bigint")
                .fetch_one(&mut *conn)
                .await
            {
                Ok(Some(_)) => {
                    tracing::warn!(
                        "before_acquire({pool_name}): rolling back stale write transaction"
                    );
                    if let Err(e) = sqlx::query("ROLLBACK").execute(&mut *conn).await {
                        tracing::warn!(
                            "before_acquire({pool_name}): ROLLBACK failed, discarding connection: {e}"
                        );
                        return Ok(false);
                    }
                    Ok(true)
                }
                Ok(None) => Ok(true),
                Err(e) => {
                    tracing::warn!(
                        "before_acquire({pool_name}): health check failed, discarding connection: {e}"
                    );
                    Ok(false)
                }
            }
        })
    }
}
