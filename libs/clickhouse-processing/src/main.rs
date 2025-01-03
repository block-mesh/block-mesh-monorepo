mod backup;
use rayon::iter::ParallelIterator;

mod s3_utils;
mod utils;

use crate::s3_utils::download_file_from_s3;
use crate::utils::{
    file_date, is_exists, process_raw, read_files_from_dir, read_lson, write_to_file_ljson,
};
use clap::Parser;
use rayon::iter::IntoParallelRefIterator;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct CliArgs {
    #[arg(long)]
    pub bucket: String,
    #[arg(long)]
    pub key: String,
    #[arg(long)]
    pub input_dir: String,
    #[arg(long)]
    pub output_dir: String,
    #[arg(long, default_value = "-1")]
    pub limit: i32,
    #[arg(long, default_value = "false")]
    pub from_s3: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = CliArgs::parse();
    println!("args = {:#?}", args);
    if args.from_s3 {
        let output_file = format!("{}/DONE__{}", args.output_dir, args.key);
        let input_file = format!("{}/{}", args.input_dir, args.key);
        let local_key = args.key.replace("/", "_");
        download_file_from_s3(&args.bucket, &args.key, &local_key, &args.input_dir).await?;
        let date = file_date(&input_file)?.to_string();
        let raws = read_lson(&input_file)?;
        let output = process_raw(raws, args.limit, date);
        write_to_file_ljson(output, &output_file);
    } else {
        let files = read_files_from_dir(&args.input_dir)?;
        files.par_iter().for_each(|file: &PathBuf| {
            println!("file = {:?}", file);
            let input_file = file.to_str().unwrap_or_default();
            let key = file
                .file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default();
            let output_file = format!("{}/DONE__{}", args.output_dir, key);
            if is_exists(&output_file) {
                println!("file = '{}' , already exists", output_file);
            } else {
                let date = file_date(input_file).unwrap().to_string();
                let raws = read_lson(input_file).unwrap();
                let output = process_raw(raws, args.limit, date);
                write_to_file_ljson(output, &output_file);
            }
        });
    }
    Ok(())
}
