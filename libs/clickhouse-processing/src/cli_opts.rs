use clap::Parser;
use sqlx::types::chrono::NaiveDate;

#[derive(Parser, Debug)]
pub struct Process {
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
    #[arg(long)]
    pub mode: String,
    #[arg(long)]
    pub input_file: String,
    #[arg(long, default_value = "2")]
    pub format: String,
}

#[derive(Parser, Debug)]
pub struct Export {
    #[arg(long)]
    pub target_bucket: String,
    #[arg(long)]
    pub target_filename: String,
    #[arg(long)]
    pub since: NaiveDate,
    #[arg(long)]
    pub until: NaiveDate,
    #[arg(long, default_value = "10")]
    pub partitions: u8,
    #[arg(long, default_value = "1000000000")]
    pub limit: u64,
}

#[derive(Parser, Debug)]
pub enum CliOpts {
    Process(Process),
    Export(Export),
}
