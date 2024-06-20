use crate::configuration::settings::Settings;
use crate::envars::app_env_var::AppEnvVar;
use crate::envars::environment::Environment;
use crate::envars::get_env_var_or_panic::get_env_var_or_panic;
use std::env;
use std::path::Path;

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let environment: Environment = get_env_var_or_panic(AppEnvVar::AppEnvironment)
        .try_into()
        .unwrap_or_else(|_| panic!("Failed to parse {}", AppEnvVar::AppEnvironment));
    let cwd = env::current_dir().expect("Failed to determine current directory");
    let configuration_directory = cwd
        .join("libs")
        .join("block-mesh-manager")
        .join("configuration");
    let configuration_file_path = configuration_directory.join("base.yaml");

    match Path::new(configuration_file_path.to_str().unwrap_or_default()).is_file() {
        true => tracing::info!(
            "Found configuration file at {}",
            configuration_file_path.to_str().unwrap_or_default()
        ),
        false => {
            tracing::error!(
                "Couldn't find configuration file at {} , cwd = {}",
                configuration_file_path.to_str().unwrap_or_default(),
                cwd.display()
            );
            return Err(config::ConfigError::NotFound(
                configuration_file_path
                    .to_str()
                    .unwrap_or_default()
                    .to_string(),
            ));
        }
    }

    config::Config::builder()
        .add_source(config::File::from(configuration_file_path).required(true))
        .add_source(
            config::File::from(
                configuration_directory.join(format!("{}.yaml", environment.as_str())),
            )
            .required(true),
        )
        .add_source(config::Environment::with_prefix("APP").separator("_"))
        .set_override_option("application.port", env::var("PORT").ok())?
        .build()?
        .try_deserialize()
}
