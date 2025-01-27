use crate::cli_opts::Export;
use block_mesh_common::date::date_range;
use clickhouse::Client;
use std::env;
use std::sync::Arc;

pub async fn export(args: Export) -> anyhow::Result<()> {
    let clickhouse_client = Arc::new(
        Client::default()
            .with_url(env::var("PROD_CLICKHOUSE_URL")?)
            .with_user(env::var("PROD_CLICKHOUSE_USER")?)
            .with_password(env::var("PROD_CLICKHOUSE_PASSWORD")?)
            .with_option("async_insert", "1")
            .with_option("wait_for_async_insert", "0"),
    );

    let aws_id = env::var("AWS_ACCESS_KEY_ID")?;
    let aws_secret = env::var("AWS_SECRET_ACCESS_KEY")?;
    for (since, until) in date_range(&args.since, &args.until) {
        println!("Starting {since} => {until}");
        let s3_output = format!(
            "{}/{}_{since}_{until}_{{_partition_id}}.json.gz",
            args.target_bucket, args.target_filename
        );
        let partitions = args.partitions;
        let limit = args.limit;
        let query = format!(
            r#"
            INSERT INTO FUNCTION
            s3('{s3_output}', '{aws_id}', '{aws_secret}', 'JSONEachRow')
            PARTITION BY rand() % {partitions}
            SELECT user_name, origin_id, link, tweet, event_date, reply, retweet, like
            FROM data_sinks_clickhouse
            WHERE event_date >= '{since}' and event_date <= '{until}'
            LIMIT {limit}
            SETTINGS max_threads = 1, max_insert_threads = 1
        "#
        );
        let _ = clickhouse_client
            .query(&query)
            .with_option("wait_end_of_query", "1")
            .execute()
            .await;
    }
    Ok(())
}
