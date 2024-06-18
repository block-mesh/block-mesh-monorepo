use crate::domain::rpc::Rpc;
use sqlx::{Postgres, Transaction};

#[tracing::instrument(name = "get_all_rpcs", skip(transaction), level = "trace", ret, err)]
pub(crate) async fn get_all_rpcs(
    transaction: &mut Transaction<'_, Postgres>,
) -> anyhow::Result<Vec<Rpc>> {
    let rpcs = sqlx::query_as!(
        Rpc,
        r#"SELECT
           id,
           name,
           token,
           host,
           created_at
           FROM rpcs
        "#,
    )
    .fetch_all(&mut **transaction)
    .await?;
    Ok(rpcs)
}
