use anyhow::anyhow;
use block_mesh_common::interfaces::server_api::FeedElement;
use clickhouse::Client;
use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDate;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    pub user_name: String,
    pub origin_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, clickhouse::Row)]
pub struct Output {
    pub user: String,
    pub id: String,
    pub link: String,
    pub tweet: String,
    pub date: String,
    pub reply: String,
    pub retweet: String,
    pub like: String,
}

impl Output {
    pub fn merge(element: FeedElement, data_sink: DataSinkClickHouse) -> Self {
        Self {
            user: element.user_name,
            id: element.id,
            link: element.link,
            tweet: element.tweet.unwrap_or_default(),
            date: data_sink.created_at,
            reply: element.reply.unwrap_or_default().to_string(),
            retweet: element.retweet.unwrap_or_default().to_string(),
            like: element.like.unwrap_or_default().to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, clickhouse::Row)]
pub struct DataSinkClickHouse {
    #[serde(with = "clickhouse::serde::uuid")]
    pub id: Uuid,
    #[serde(with = "clickhouse::serde::uuid")]
    pub user_id: Uuid,
    pub origin: String,
    pub origin_id: String,
    pub user_name: String,
    pub link: String,
    pub created_at: String,
    pub updated_at: String,
    pub raw: String,
    pub reply: String,
    pub retweet: String,
    pub like: String,
    pub tweet: String,
}

pub fn read_csv_file<T>(path: &str) -> Vec<T>
where
    T: for<'a> Deserialize<'a>,
{
    let mut reader = ReaderBuilder::new().from_path(path).unwrap();
    let mut records: Vec<T> = Vec::with_capacity(3_000_000);
    for result in reader.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: T = result.unwrap();
        records.push(record);
    }
    records
}

pub fn write_to_csv_file<T>(records: Vec<T>, path: &str)
where
    T: Serialize,
{
    let mut wtr = csv::Writer::from_path(path).unwrap();
    for record in records {
        wtr.serialize(record).unwrap();
    }
    wtr.flush().unwrap();
}

pub fn query_builder(records: &[Record], date: NaiveDate) -> String {
    let vec: Vec<String> = records
        .iter()
        .map(|i| format!("('{}', '{}')", i.user_name, i.origin_id))
        .collect();
    let vec_str = vec.join(",");
    format!(
        r#"
            SELECT ?fields
            FROM data_sinks_clickhouse
            WHERE event_date = '{date}'
            AND (user_name, origin_id) in ({vec_str})
    "#
    )
}

pub async fn process_chunk(
    clickhouse_client: Arc<Client>,
    date: NaiveDate,
    records: &[Record],
) -> anyhow::Result<Vec<DataSinkClickHouse>> {
    let query_str = query_builder(records, date);
    let data = clickhouse_client
        .query(&query_str)
        .fetch_all::<DataSinkClickHouse>()
        .await
        .map_err(|e| {
            eprintln!("process_chunk {}", e);
            anyhow!(e.to_string())
        })?;
    Ok(data)
}

pub fn write_chunk(records: Vec<DataSinkClickHouse>, index: u64, date: &NaiveDate) {
    let mut wtr = csv::Writer::from_path(format!("./CSV_OUTPUT/{}_{}.csv", date, index)).unwrap();
    for record in records {
        wtr.serialize(record).unwrap();
    }
    wtr.flush().unwrap();
}
