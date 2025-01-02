use anyhow::anyhow;
use aws_config::meta::region::RegionProviderChain;
use aws_config::Region;
use aws_sdk_s3::Client;
use std::io::Write;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub async fn download_file_from_s3(
    bucket: &str,
    key: &str,
    local_key: &str,
    dir_path: &str,
) -> anyhow::Result<()> {
    let region_provider = RegionProviderChain::first_try(Region::new("us-west-2"));
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);
    let file_path = format!("{}/{}", dir_path, local_key);
    let head_object = client.head_object().bucket(bucket).key(key).send().await?;
    let total_size = head_object.content_length().unwrap_or_default();
    match client.get_object().bucket(bucket).key(key).send().await {
        Ok(output) => {
            let mut file = File::create(&file_path).await?;
            let mut stream = output.body;
            let mut downloaded: u64 = 0;
            println!("Starting download...");

            // Stream the data
            while let Some(chunk) = stream.next().await {
                let data = chunk?;
                file.write_all(&data).await?;
                downloaded += data.len() as u64;
                let progress = downloaded as f64 / total_size as f64 * 100.0;
                print!("\rProgress: {:.2}%", progress);
                let _ = std::io::stdout().flush();
            }
            file.flush().await?;
            println!("File downloaded successfully to {}", file_path);
        }
        Err(e) => {
            eprintln!("[download_file_from_s3] Error {}", e);
            return Err(anyhow!(e));
        }
    }

    Ok(())
}
