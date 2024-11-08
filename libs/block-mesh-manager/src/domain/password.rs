use anyhow::anyhow;
use secret::Secret;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Password(Secret<String>);

impl Password {
    pub fn new(s: String) -> anyhow::Result<Self> {
        if s.is_empty() {
            Err(anyhow!("Password cannot be empty"))
        } else if s.contains(' ') {
            Err(anyhow!("Password cannot contain spaces"))
        } else if s.chars().all(char::is_alphanumeric) {
            Err(anyhow!("Password cannot contain alphanumeric characters"))
        } else if s.len() < 8 {
            Err(anyhow!("Password cannot contain less than 8 characters"))
        } else {
            Ok(Self(Secret::from(s)))
        }
    }
}
