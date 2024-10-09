use block_mesh_common::interfaces::ip_data::IPData;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[allow(dead_code)]
pub async fn enrich_ip_address(
    transaction: &mut Transaction<'_, Postgres>,
    id: Uuid,
    ip_data: &IPData,
) -> anyhow::Result<()> {
    let ip_geolocate_response = ip_data.ip_geolocate_response.as_ref();
    sqlx::query!(
        r#"UPDATE ip_addresses SET
           latitude = $1 ,
           longitude = $2 ,
           city = $3 ,
           region = $4 ,
           country = $5 ,
           timezone = $6 ,
           isp = $7,
           enriched = $8
           WHERE id = $9"#,
        ip_geolocate_response
            .as_ref()
            .map(|i| i.latitude.parse::<f64>().unwrap_or_default()),
        ip_geolocate_response.map(|i| i.longitude.parse::<f64>().unwrap_or_default()),
        ip_geolocate_response.map(|i| i.city.clone()),
        ip_geolocate_response.map(|i| i.region.clone()),
        ip_geolocate_response.map(|i| i.country.clone()),
        ip_geolocate_response.map(|i| i.timezone.clone()),
        ip_geolocate_response.map(|i| i.isp.clone()),
        true,
        id
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
