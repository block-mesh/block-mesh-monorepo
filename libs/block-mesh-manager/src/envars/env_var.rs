use crate::domain::secret::Secret;
use std::fmt::Display;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum EnvVar {
    Secret(Secret<String>),
    Public(String),
}

impl PartialEq<EnvVar> for String {
    fn eq(&self, other: &EnvVar) -> bool {
        match other {
            EnvVar::Secret(secret) => *self == *secret.as_ref(),
            EnvVar::Public(public) => *self == *public,
        }
    }
}

impl From<EnvVar> for PathBuf {
    fn from(env_var: EnvVar) -> Self {
        match env_var {
            EnvVar::Secret(secret) => PathBuf::from(secret.as_ref()),
            EnvVar::Public(public) => PathBuf::from(public),
        }
    }
}

impl AsRef<Secret<String>> for EnvVar {
    fn as_ref(&self) -> &Secret<String> {
        match self {
            EnvVar::Secret(secret) => secret,
            EnvVar::Public(public) => panic!("{} is not a secret", public),
        }
    }
}

impl AsRef<String> for EnvVar {
    fn as_ref(&self) -> &String {
        match self {
            EnvVar::Secret(secret) => panic!("{} is a secret", secret.as_ref()),
            EnvVar::Public(public) => public,
        }
    }
}

impl Display for EnvVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnvVar::Secret(_) => {
                write!(f, "{:?}", <EnvVar as AsRef<Secret<String>>>::as_ref(self))
            }
            EnvVar::Public(_) => write!(f, "{:?}", <EnvVar as AsRef<String>>::as_ref(self)),
        }
    }
}
