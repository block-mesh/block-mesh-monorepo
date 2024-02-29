use crate::envars::app_env_var::AppEnvVar;
use enum_iterator::all;

pub fn load_dotenv() {
    let dotenv_file = format!(
        ".env.{}",
        std::env::var("APP_ENVIRONMENT").unwrap_or("local".into())
    );
    tracing::info!("Loading {}", dotenv_file);
    dotenv::from_filename(&dotenv_file).ok();
    for key in all::<AppEnvVar>().collect::<Vec<_>>() {
        let val = std::env::var(key.to_string());
        if val.is_err() {
            continue;
        }
        tracing::info!("{}={}", key, val.unwrap());
    }
}
