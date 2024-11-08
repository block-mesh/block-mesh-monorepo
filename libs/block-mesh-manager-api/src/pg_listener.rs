use block_mesh_common::interfaces::db_messages::InvalidateApiCache;
use block_mesh_common::interfaces::server_api::{CheckTokenResponseMap, GetTokenResponseMap};
use sqlx::error::Error;
use sqlx::postgres::PgListener;
use sqlx::Pool;
use sqlx::Postgres;
use tracing::error;
use uuid::Uuid;

#[allow(dead_code)]
#[tracing::instrument(name = "start_listening", skip_all, err)]
pub async fn start_listening(
    pool: Pool<Postgres>,
    channels: Vec<&str>,
    check_token_map: CheckTokenResponseMap,
    get_token_map: GetTokenResponseMap,
) -> Result<(), Error> {
    let mut listener = PgListener::connect_with(&pool).await?;
    listener.listen_all(channels).await?;
    loop {
        while let Ok(Some(notification)) = listener.try_recv().await {
            let string = notification.payload().to_owned();
            if let Ok(payload) = serde_json::from_str::<InvalidateApiCache>(&string) {
                let found = find_check_token_map_key(&check_token_map, &payload.email);
                if let Some(found) = found {
                    check_token_map.remove(&found);
                }
                let found = find_get_token_map_key(&get_token_map, &payload.email);
                if let Some(found) = found {
                    get_token_map.remove(&found);
                }
            } else {
                error!("Failed to deserialize {:?}", string);
            }
        }
    }
}

#[tracing::instrument(name = "find_check_token_map_key", skip_all)]
pub fn find_check_token_map_key(
    check_token_map: &CheckTokenResponseMap,
    email: &str,
) -> Option<(String, Uuid)> {
    match check_token_map.iter().find(|i| i.key().0 == email) {
        Some(found) => Some(found.key().clone()),
        None => None,
    }
}

#[tracing::instrument(name = "find_get_token_map_key", skip_all)]
pub fn find_get_token_map_key(
    get_token_map: &GetTokenResponseMap,
    email: &str,
) -> Option<(String, String)> {
    match get_token_map.iter().find(|i| i.key().0 == email) {
        Some(found) => Some(found.key().clone()),
        None => None,
    }
}
