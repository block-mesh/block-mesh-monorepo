mod backup;
mod utils;

use crate::utils::{write_to_csv_file, DataSinkClickHouse, Output};
use anyhow::anyhow;
use clap::Parser;
use csv::{ReaderBuilder, StringRecord};
use twitter_scraping_helper::feed_element_try_from;

#[derive(Parser, Debug)]
pub struct CliArgs {
    #[arg(long)]
    pub input_file: String,
    #[arg(long)]
    pub output_file: String,
    #[arg(long, default_value = "-1")]
    pub limit: i32,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = CliArgs::parse();
    println!("args = {:#?}", args);
    if args.input_file == args.output_file {
        let msg = "Input and output file cannot be the same";
        eprintln!("{}", msg);
        return Err(anyhow!(msg));
    }
    // let inputs = read_csv_file(&args.input_file);

    let mut reader = ReaderBuilder::new()
        .buffer_capacity(64_000_000)
        .from_path(args.input_file)
        .unwrap();
    let mut string_record = StringRecord::new();
    let mut output: Vec<Output> = Vec::with_capacity(1_000_000);

    let mut count = 0;
    loop {
        if count % 1_000 == 0 {
            println!("count = {}", count);
        }
        reader.read_record(&mut string_record).unwrap();
        let record: DataSinkClickHouse = string_record.deserialize(None).unwrap();
        let element = feed_element_try_from(&record.raw, &record.origin).map_err(|e| {
            eprintln!("error = {}", e);
            anyhow!(e.to_string())
        })?;
        output.push(Output::merge(element, record));
        if args.limit > 0 && count > args.limit {
            break;
        }
        count += 1;
    }

    write_to_csv_file(output, &args.output_file);
    Ok(())
}
