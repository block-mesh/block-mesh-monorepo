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

impl CollectorData {
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
