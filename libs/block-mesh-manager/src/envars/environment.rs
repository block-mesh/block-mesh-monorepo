use crate::envars::env_var::EnvVar;
use std::str::FromStr;

pub enum Environment {
    Local,
    Staging,
    Production,
    Test,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Staging => "staging",
            Environment::Production => "production",
            Environment::Test => "test",
        }
    }
}

impl TryFrom<EnvVar> for Environment {
    type Error = String;

    fn try_from(value: EnvVar) -> Result<Self, Self::Error> {
        match value {
            EnvVar::Secret(secret) => Environment::from_str(secret.as_ref()),
            EnvVar::Public(public) => Environment::from_str(&public),
        }
    }
}

impl FromStr for Environment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Environment::Local),
            "staging" => Ok(Environment::Staging),
            "production" => Ok(Environment::Production),
            "test" => Ok(Environment::Test),
            other => Err(format!("{} is not a valid environment", other)),
        }
    }
}
