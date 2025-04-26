use anyhow::{Context, anyhow};
use chrono::{DateTime, NaiveDate, Utc};
use database_utils::utils::option_uuid::OptionUuid;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct CollectorDailyStats {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub day: NaiveDate,
    pub count: i32,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct CollectorDailyStatsTmp {
    pub id: OptionUuid,
    pub created_at: Option<DateTime<Utc>>,
    pub day: Option<NaiveDate>,
    pub count: Option<i32>,
}

impl CollectorDailyStats {
    #[tracing::instrument(name = "update_daily_stats", skip_all, err)]
    pub async fn update_daily_stats(
        transaction: &mut Transaction<'_, Postgres>,
        add_to_count: i32,
    ) -> anyhow::Result<()> {
        let now = Utc::now();
        let day = now.date_naive();
        sqlx::query!(
            r#"
            UPDATE collector_daily_stats
            SET count = count + $2  
            WHERE (day) = ($1)
            "#,
            day,
            add_to_count
        )
        .execute(&mut **transaction)
        .await?;
        Ok(())
    }
    #[tracing::instrument(name = "get_or_create_collector_daily_stats", skip_all, err)]
    pub async fn get_or_create_collector_daily_stats(
        transaction: &mut Transaction<'_, Postgres>,
    ) -> anyhow::Result<CollectorDailyStats> {
        let now = Utc::now();
        let day = now.date_naive();
        let id = Uuid::new_v4();
        let collector_daily_stats = sqlx::query_as!(
            CollectorDailyStatsTmp,
            r#"
                WITH extant AS (
                	SELECT id,created_at,day,count
                	FROM collector_daily_stats
                	WHERE (day) = ($3)
                ),
                inserted AS (
                INSERT INTO collector_daily_stats (id,created_at,day, count)
                SELECT $1,$2,$3,$4
                WHERE
                	NOT EXISTS (SELECT	FROM extant)
                	RETURNING id,created_at,day,count
                )
                SELECT id,created_at,day,count
                FROM inserted
                UNION ALL
                SELECT id,created_at,day,count
                FROM extant;
                "#,
            id,
            now.clone(),
            day,
            0,
        )
        .fetch_one(&mut **transaction)
        .await?;
        Ok(CollectorDailyStats {
            id: collector_daily_stats.id.expect("Missing ID"),
            created_at: collector_daily_stats
                .created_at
                .expect("MISSING TIMESTAMP CREATED_AT"),
            day: collector_daily_stats.day.expect("MISSING DAY"),
            count: collector_daily_stats.count.expect("MISSING COUNT"),
        })
    }
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct CollectorData {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub source: String,
    pub data: Value,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExportData {
    /*
    • Columns: brand, model
     */
    pub asin: String,
    pub url: String,
    pub brand: String,
    pub product_price: f64,
    pub review_count: i64,
    pub rating: f64,
    pub in_stock: String,
    pub last_seen: NaiveDate,
}

impl CollectorData {
    #[tracing::instrument(name = "extract_for_export", skip_all, err)]
    pub fn extract_for_export(&self) -> anyhow::Result<ExportData> {
        let data = self.data.as_object().context("data isn't an object")?;
        let data = data
            .get("data")
            .ok_or(anyhow!("(1) Cant find data"))?
            .as_object()
            .ok_or(anyhow!("(2) Cant find data"))?;
        let asin = data
            .get("asin")
            .ok_or(anyhow!("(1) Cant find asin"))?
            .as_str()
            .ok_or(anyhow!("(2) Cant find asin"))?
            .to_string();
        let url = format!("https://www.amazon.com/gp/product/{}", asin);
        let product_details = data
            .get("product_details")
            .ok_or(anyhow!("(1) Cant find product_details"))?
            .as_object()
            .ok_or(anyhow!("(2) Cant find product_details"))?;
        let brand = product_details
            .get("Brand")
            .ok_or(anyhow!("Cant find brand"))?
            .as_str()
            .unwrap_or_default()
            .to_string();
        tracing::info!(" data.get(product_price) = {:?}", data.get("product_price"));
        let product_price = data
            .get("product_price")
            .ok_or(anyhow!("(1) Cant find product price"))?
            .as_str()
            .ok_or(anyhow!("(2) Cant find product price"))?
            .parse::<f64>()?;
        let in_stock = data
            .get("product_availability")
            .ok_or(anyhow!("(1) Cant find product_availability"))?
            .as_str()
            .ok_or(anyhow!("(2) Cant find product_availability"))?
            .to_string();
        let review_count = data
            .get("product_num_ratings")
            .ok_or(anyhow!("(1) Cant find product_num_ratings"))?
            .as_i64()
            .ok_or(anyhow!("(2) Cant find product_num_ratings"))?;
        let rating = data
            .get("product_star_rating")
            .ok_or(anyhow!("(1) Cant find product_star_rating"))?
            .as_str()
            .ok_or(anyhow!("(2) Cant find product_star_rating"))?
            .parse::<f64>()?;
        let last_seen = self.created_at.date_naive();
        Ok(ExportData {
            asin,
            url,
            brand,
            product_price,
            in_stock,
            review_count,
            rating,
            last_seen,
        })
    }

    #[tracing::instrument(name = "get_day_data", skip_all, err)]
    pub async fn get_day_data(
        transaction: &mut Transaction<'_, Postgres>,
        day: NaiveDate,
        limit: i64,
    ) -> anyhow::Result<Vec<Self>> {
        Ok(sqlx::query_as!(
            CollectorData,
            r#"
            SELECT
            id, created_at, source, data
            FROM collector_datas
            WHERE DATE(created_at) = $1
            LIMIT $2
            "#,
            day,
            limit
        )
        .fetch_all(&mut **transaction)
        .await?)
    }

    #[tracing::instrument(name = "create_new_collector_data", skip_all, err)]
    pub async fn create_new_collector_data(
        transaction: &mut Transaction<'_, Postgres>,
        source: &str,
        data: &Value,
    ) -> anyhow::Result<()> {
        let now = Utc::now();
        let id = Uuid::new_v4();
        sqlx::query!(
            r#"
            INSERT INTO collector_datas
            (id, created_at, source, data)
            VALUES ($1, $2, $3, $4)
        "#,
            id,
            now,
            source,
            data
        )
        .execute(&mut **transaction)
        .await?;
        Ok(())
    }
}
