use crate::domain::secret::Secret;
use crate::envars::app_env_var::AppEnvVar;
use crate::envars::env_var::EnvVar;

pub fn get_env_var_or_panic(key: AppEnvVar) -> EnvVar {
    match std::env::var(key.to_string()) {
        Ok(val) => {
            let value = if key == AppEnvVar::WalletMnemonic || key == AppEnvVar::DatabaseUrl {
                EnvVar::Secret(Secret::from(val))
            } else {
                EnvVar::Public(val)
            };
            tracing::info!("{} is set to: {}", key, value);
            value
        }
        Err(e) => {
            tracing::error!("{} is not set: {}", key, e);
            panic!("{} is not set: {}", key, e)
        }
    }
}
