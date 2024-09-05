use enum_iterator::Sequence;
use std::fmt::Display;
use std::ops::Deref;

#[derive(Debug, PartialEq, Sequence, Clone)]
pub enum AppEnvVar {
    MailgunSendKey,
    AppEnvironment,
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
            AppEnvVar::AppEnvironment => "APP_ENVIRONMENT",
            AppEnvVar::DatabaseUrl => "DATABASE_URL",
            AppEnvVar::MailgunSendKey => "MAILGUN_SEND_KEY",
        }
    }
}

impl Display for AppEnvVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppEnvVar::AppEnvironment => write!(f, "APP_ENVIRONMENT"),
            AppEnvVar::DatabaseUrl => write!(f, "DATABASE_URL"),
            AppEnvVar::MailgunSendKey => write!(f, "MAILGUN_SEND_KEY"),
        }
    }
}
