mod backup;

mod s3_utils;
mod utils;

use crate::s3_utils::download_file_from_s3;
use crate::utils::{file_date, process_raw, read_lson, write_to_file_ljson};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct CliArgs {
    #[arg(long)]
    pub bucket: String,
    #[arg(long)]
    pub key: String,
    #[arg(long)]
    pub dir: String,
    #[arg(long, default_value = "-1")]
    pub limit: i32,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = CliArgs::parse();
    println!("args = {:#?}", args);
    let output_file = format!("{}/DONE__{}", args.dir, args.key);
    let input_file = format!("{}/{}", args.dir, args.key);
    let local_key = args.key.replace("/", "_");
    download_file_from_s3(&args.bucket, &args.key, &local_key, &args.dir).await?;
    let date = file_date(&input_file)?.to_string();
    let raws = read_lson(&input_file)?;
    let output = process_raw(raws, args.limit, date);
    write_to_file_ljson(output, &output_file);
    Ok(())
}
