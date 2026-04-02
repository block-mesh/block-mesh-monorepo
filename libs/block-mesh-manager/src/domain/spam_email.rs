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
            if Self::matches_domain(email_domain, &domain.domain) {
                return Err(anyhow!(
                    "Spam email found {} | {}",
                    email_domain,
                    domain.domain
                ));
            }
        }
        Ok(())
    }

    fn matches_domain(email_domain: &str, blocked_domain: &str) -> bool {
        email_domain == blocked_domain
            || email_domain
                .strip_suffix(blocked_domain)
                .is_some_and(|prefix| prefix.ends_with('.'))
    }
}

#[cfg(test)]
mod tests {
    use super::SpamEmail;
    use time::OffsetDateTime;
    use uuid::Uuid;

    fn spam_email(domain: &str) -> SpamEmail {
        SpamEmail {
            id: Uuid::nil(),
            domain: domain.to_string(),
            created_at: OffsetDateTime::UNIX_EPOCH,
            updated_at: OffsetDateTime::UNIX_EPOCH,
        }
    }

    #[test]
    fn rejects_exact_blocked_domain() {
        let result = SpamEmail::check_domains("yopmail.com", vec![spam_email("yopmail.com")]);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_blocked_subdomain() {
        let result = SpamEmail::check_domains("foo.yopmail.com", vec![spam_email("yopmail.com")]);

        assert!(result.is_err());
    }

    #[test]
    fn allows_non_matching_suffix_like_domain() {
        let result = SpamEmail::check_domains("notyopmail.com", vec![spam_email("yopmail.com")]);

        assert!(result.is_ok());
    }
}
