mod utils;

use crate::utils::{process_chunk, read_csv_file, write_chunk, Record};
use clickhouse::Client;
use sqlx::types::chrono::{NaiveDate, Utc};
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let clickhouse_client = Arc::new(
        Client::default()
            .with_url(env::var("PROD_CLICKHOUSE_URL").unwrap())
            .with_user(env::var("PROD_CLICKHOUSE_USER").unwrap())
            .with_password(env::var("PROD_CLICKHOUSE_PASSWORD").unwrap())
            .with_option("async_insert", "1")
            .with_option("wait_for_async_insert", "0"),
    );

    let mut records = read_csv_file("./CSV/backup-2024-12-11.csv");
    let total_records = records.len();
    println!("records.len = {}", total_records);
    let date = NaiveDate::from_ymd_opt(2024, 12, 11).unwrap();
    let mut index = 0;
    let mut acc = 0;
    while !records.is_empty() {
        let clickhouse_client = clickhouse_client.clone();
        let test_records: Vec<Record> = records.drain(0..999).collect();
        let mut success = false;
        while !success {
            let clickhouse_client = clickhouse_client.clone();
            if let Ok(db_records) = process_chunk(clickhouse_client, date, &test_records).await {
                acc += db_records.len();
                let now = Utc::now();
                println!(
                    "[{}] stats: index = {} | db_records.len() = {} | acc = {} | total = {}",
                    now,
                    index,
                    db_records.len(),
                    acc,
                    total_records
                );
                write_chunk(db_records, index, &date);
                index += 1;
                success = true;
            }
        }
    }
    Ok(())
}
