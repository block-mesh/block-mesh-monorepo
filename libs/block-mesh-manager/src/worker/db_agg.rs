use crate::database::aggregate::update_aggregate::update_aggregate_bulk;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpdateAggMessage {
    pub(crate) id: Uuid,
    pub(crate) value: Value,
}

pub async fn db_agg(
    pool: PgPool,
    mut rx: tokio::sync::mpsc::Receiver<UpdateAggMessage>,
) -> Result<(), anyhow::Error> {
    let mut queries: Vec<String> = Vec::with_capacity(100);
    let mut calls: HashMap<Uuid, Value> = HashMap::new();
    let mut count = 0;
    while let Some(query) = rx.recv().await {
        calls.insert(query.id, query.value);
        count += 1;
        if count == 100 {
            if let Ok(mut transaction) = pool.begin().await {
                match update_aggregate_bulk(&mut transaction, &mut calls).await {
                    Ok(_) => {}
                    Err(e) => tracing::error!("ERROR update_aggregate_bulk {}", e),
                }
                match transaction.commit().await {
                    Ok(_) => {}
                    Err(e) => tracing::error!("ERROR update_aggregate_bulk commit {}", e),
                }
                calls.clear();
                queries.clear();
                count = 0;
            }
        }
    }
    Ok(())
}
