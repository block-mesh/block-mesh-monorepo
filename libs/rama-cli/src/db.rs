use chrono::{DateTime, Utc};
use database_utils::utils::option_uuid::OptionUuid;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct RamaId {
    pub id: Uuid,
    pub email: String,
    pub api_token: String,
    pub ja3: String,
    pub ja4: String,
    pub ja4h: String,
    pub ip: String,
    pub created_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct RamaIdTmp {
    pub id: OptionUuid,
    pub email: Option<String>,
    pub api_token: Option<String>,
    pub ja3: Option<String>,
    pub ja4: Option<String>,
    pub ja4h: Option<String>,
    pub ip: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

#[allow(clippy::too_many_arguments)]
pub async fn get_or_create_rama_id(
    transaction: &mut Transaction<'_, Postgres>,
    email: &str,
    api_token: &str,
    ja3: &str,
    ja4: &str,
    ja4h: &str,
    ip: &str,
) -> anyhow::Result<RamaId> {
    let uuid = Uuid::new_v4();
    let now = Utc::now();
    let idtmp = sqlx::query_as!(
        RamaIdTmp,
        r#"
WITH extant AS (
	SELECT id, email, api_token, ja3, ja4, ja4h, ip, created_at
	FROM rama_ids
	WHERE (email, api_token, ja4 , ip) = ($2, $3, $5, $7)
),
inserted AS (
INSERT INTO rama_ids ( id, email, api_token, ja3, ja4, ja4h , ip, created_at)
SELECT $1, $2, $3, $4 , $5 , $6, $7, $8
WHERE
	NOT EXISTS (SELECT	FROM extant)
	RETURNING  id, email, api_token, ja3, ja4, ja4h, ip, created_at
)
SELECT id, email, api_token, ja3, ja4, ja4h, ip, created_at
FROM inserted
UNION ALL
SELECT id, email, api_token, ja3, ja4, ja4h, ip, created_at
FROM extant;
"#,
        uuid,
        email,
        api_token,
        ja3,
        ja4,
        ja4h,
        ip,
        now
    )
    .fetch_one(&mut **transaction)
    .await?;
    let id = RamaId {
        id: idtmp.id.expect("MISSING ID"),
        email: idtmp.email.expect("MISSING EMAIL"),
        api_token: idtmp.api_token.expect("MISSING API TOKEN"),
        ja3: idtmp.ja3.expect("MISSING JA3"),
        ja4: idtmp.ja4.expect("MISSING JA4"),
        ja4h: idtmp.ja4h.expect("MISSING JA4H"),
        ip: idtmp.ip.expect("MISSING IP"),
        created_at: idtmp.created_at.expect("MISSING TIMESTAMP CREATED_AT"),
    };
    Ok(id)
}
