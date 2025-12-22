use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct SpamEmail {
    pub id: Uuid,
    pub domain: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
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
