use enum_iterator::Sequence;
use std::fmt::Display;
use std::ops::Deref;

#[derive(Debug, PartialEq, Sequence, Clone)]
pub enum AppEnvVar {
    BasePath,
    AppEnvironment,
    AssetsDir,
    FullBasePath,
    ViteDir,
    WalletMnemonic,
    DatabaseUrl,
}

impl PartialEq<AppEnvVar> for String {
    fn eq(&self, other: &AppEnvVar) -> bool {
        *self == <str as AsRef<str>>::as_ref(other)
    }
}

impl Deref for AppEnvVar {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            AppEnvVar::BasePath => "BASE_PATH",
            AppEnvVar::AppEnvironment => "APP_ENVIRONMENT",
            AppEnvVar::AssetsDir => "ASSETS_DIR",
            AppEnvVar::FullBasePath => "FULL_BASE_PATH",
            AppEnvVar::ViteDir => "VITE_DIR",
            AppEnvVar::WalletMnemonic => "WALLET_MNEMONIC",
            AppEnvVar::DatabaseUrl => "DATABASE_URL",
        }
    }
}

impl Display for AppEnvVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppEnvVar::BasePath => write!(f, "BASE_PATH"),
            AppEnvVar::AppEnvironment => write!(f, "APP_ENVIRONMENT"),
            AppEnvVar::AssetsDir => write!(f, "ASSETS_DIR"),
            AppEnvVar::FullBasePath => write!(f, "FULL_BASE_PATH"),
            AppEnvVar::ViteDir => write!(f, "VITE_DIR"),
            AppEnvVar::WalletMnemonic => write!(f, "WALLET_MNEMONIC"),
            AppEnvVar::DatabaseUrl => write!(f, "DATABASE_URL"),
        }
    }
}
