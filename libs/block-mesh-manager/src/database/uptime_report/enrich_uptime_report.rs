use block_mesh_common::interfaces::ip_data::IPData;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "enrich_uptime_report", skip(transaction), ret, err)]
pub(crate) async fn enrich_uptime_report(
    transaction: &mut Transaction<'_, Postgres>,
    uptime_id: Uuid,
    ip_data: IPData,
) -> anyhow::Result<()> {
    let ip_geolocate_response = ip_data.ip_geolocate_response.as_ref();
    sqlx::query!(
        r#"UPDATE uptime_reports SET
           ip = $1 ,
           latitude = $2 ,
           longitude = $3 ,
           city = $4 ,
           region = $5 ,
           country = $6 ,
           timezone = $7 ,
           isp = $8
           WHERE id = $9"#,
        ip_data.ip(),
        ip_geolocate_response
            .as_ref()
            .map(|i| i.latitude.parse::<f64>().unwrap_or_default()),
        ip_geolocate_response.map(|i| i.longitude.parse::<f64>().unwrap_or_default()),
        ip_geolocate_response.map(|i| i.city.clone()),
        ip_geolocate_response.map(|i| i.region.clone()),
        ip_geolocate_response.map(|i| i.country.clone()),
        ip_geolocate_response.map(|i| i.timezone.clone()),
        ip_geolocate_response.map(|i| i.isp.clone()),
        uptime_id
    )
    .execute(&mut **transaction)
    .await?;
    Ok(())
}
