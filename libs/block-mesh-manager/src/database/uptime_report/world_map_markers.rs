use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};

#[derive(Debug, Serialize, Deserialize)]
pub struct Marker {
    pub city: Option<String>,
    pub country: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub count: Option<i64>,
}

#[tracing::instrument(name = "world_map_markers", skip(transaction), ret, err)]
pub(crate) async fn world_map_markers(
    transaction: &mut Transaction<'_, Postgres>,
) -> anyhow::Result<Vec<Marker>> {
    let markers = sqlx::query_as!(
        Marker,
        r#"
            SELECT
                city,
                country,
                MIN(latitude) AS latitude,
                MIN(longitude) AS longitude,
                count(*) AS count
            FROM
            	ip_addresses
            WHERE
            	city IS NOT NULL
            	AND city != ''
            	AND country != ''
            	AND country IS NOT NULL
            	AND latitude IS NOT NULL
            	AND longitude IS NOT NULL
            GROUP BY
            	city,
            	country
        "#,
    )
    .fetch_all(&mut **transaction)
    .await?;
    Ok(markers)
}
