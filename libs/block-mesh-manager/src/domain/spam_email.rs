use anyhow::anyhow;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct SpamEmail {
    pub id: Uuid,
    pub domain: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl SpamEmail {
    pub fn check_domains(email_domain: &str, spam_emails: Vec<SpamEmail>) -> anyhow::Result<()> {
        for domain in spam_emails {
            if email_domain.contains(&domain.domain) {
                return Err(anyhow!(
                    "Spam email found {} | {}",
                    email_domain,
                    domain.domain
                ));
            }
        }
        Ok(())
    }
}
