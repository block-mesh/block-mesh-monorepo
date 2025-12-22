use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, FromRow)]
#[allow(dead_code)]
pub struct Id {
    pub id: Uuid,
}
