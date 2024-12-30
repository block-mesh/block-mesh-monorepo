use anyhow::anyhow;
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
pub struct DataSinkClickHouse {
    #[serde(with = "clickhouse::serde::uuid")]
    pub id: Uuid,
    #[serde(with = "clickhouse::serde::uuid")]
    pub user_id: Uuid,
    pub created_at: u64,
    pub updated_at: u64,
    pub raw: String,
    pub origin: String,
    pub origin_id: String,
    pub user_name: String,
    pub link: String,
    pub reply: u32,
    pub retweet: u32,
    pub like: u32,
    pub tweet: String,
}

pub fn read_csv_file(path: &str) -> Vec<Record> {
    let mut reader = ReaderBuilder::new().from_path(path).unwrap();
    let mut records: Vec<Record> = Vec::with_capacity(3_000_000);
    for result in reader.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Record = result.unwrap();
        records.push(record);
    }
    records
}

pub fn query_builder(records: &Vec<Record>, date: NaiveDate) -> String {
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
    records: &Vec<Record>,
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
