use clickhouse::Client;
use sqlx::types::chrono::NaiveDate;
use std::env;
use std::sync::Arc;

pub async fn process_chunk(
    _clickhouse_client: Arc<Client>,
    index: i32,
    _date: NaiveDate,
    limit: i64,
    offset: i64,
) -> anyhow::Result<()> {
    println!(
        "index = {}  | limit = {} | offset = {}",
        index, limit, offset
    );
    let mut writer = csv::WriterBuilder::new()
        .buffer_capacity(10_000)
        .from_path(format!("csv/2024-12-11_{}.csv", index))?;
    // let data = clickhouse_client
    //     .query(
    //         r#"
    //                SELECT ?fields
    //                FROM ?
    //                WHERE created_at::DATE = ?
    //                ORDER BY created_at ASC
    //                LIMIT ?
    //                OFFSET ?
    //
    //         "#,
    //     )
    //     .bind(index)
    //     .bind(Identifier("data_sinks_clickhouse"))
    //     .bind(date)
    //     .bind(limit)
    //     .bind(offset)
    //     .fetch_all::<DataSinkClickHouse>()
    //     .await
    //     .map_err(|e| {
    //         eprintln!("process_chunk {}", e);
    //         anyhow!(e.to_string())
    //     })?;
    // for i in data {
    //     let x = i.clone();
    //     // println!("{:#?}", i);
    //     match ExportData::try_from(i) {
    //         Ok(data) => {
    //             // println!("data => {:#?}", data);
    //             let _ = writer.serialize(data);
    //         }
    //         Err(e) => {
    //             eprintln!("error {}", e);
    //             eprintln!("error {:#?}", x);
    //             break;
    //         }
    //     }
    // }
    let _ = writer.flush();
    Ok(())
}

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
    let total = 10_000;
    let limit = 500;
    let mut offset = 0;
    let date = NaiveDate::from_ymd_opt(2024, 12, 11).unwrap();
    let mut jobs = vec![];
    let mut index = 0;
    while offset < total {
        let task = tokio::spawn(process_chunk(
            clickhouse_client.clone(),
            index,
            date,
            limit,
            offset,
        ));
        jobs.push(task);
        offset += limit;
        index += 1;
    }

    for (index, job) in jobs.into_iter().enumerate() {
        let _ = job.await;
        println!("Finished index = {}", index);
    }
    Ok(())
}
