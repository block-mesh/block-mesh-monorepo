use crate::email_client::client::{
    EmailClient, CONFIRM_SUBJECT, REPLY_TO, RESET_SUBJECT, SMTP_FROM,
};
use crate::email_client::confirm_email::CONFIRM_EMAIL;
use crate::email_client::reset_email::RESET_EMAIL;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message as LetterMessage, SmtpTransport, Transport};
use std::env;

impl EmailClient {
    #[tracing::instrument(name = "send_confirmation_email_smtp", skip_all, ret, err)]
    pub async fn send_confirmation_email_smtp(&self, to: &str, token: &str) -> anyhow::Result<()> {
        let smtp_password = env::var("SMTP_PASSWORD")?;
        let body = CONFIRM_EMAIL.replace(
            "{{action_url}}",
            &format!("{}/email_confirm?token={}", self.base_url, token),
        );
        let email = LetterMessage::builder()
            .from(SMTP_FROM.parse()?)
            .to(to.parse()?)
            .subject(CONFIRM_SUBJECT)
            .reply_to(REPLY_TO.parse()?)
            .header(ContentType::TEXT_HTML)
            .body(body)?;
        let creds = Credentials::new(SMTP_FROM.to_owned(), smtp_password.to_owned());
        let mailer = SmtpTransport::relay("smtp.gmail.com")?
            .credentials(creds)
            .build();
        // Send the email
        match mailer.send(&email) {
            Ok(result) => {
                tracing::info!("Email sent: {:?}", result);
                Ok(())
            }
            Err(e) => Err(e.into()),
        }
    }

    #[tracing::instrument(name = "send_reset_password_email_smtp", skip_all, ret, err)]
    pub async fn send_reset_password_email_smtp(
        &self,
        to: &str,
        token: &str,
    ) -> anyhow::Result<()> {
        let smtp_password = env::var("SMTP_PASSWORD")?;
        let body = RESET_EMAIL.replace(
            "{{action_url}}",
            &format!("{}/new_password?token={}", self.base_url, token),
        );
        let email = LetterMessage::builder()
            .from(SMTP_FROM.parse()?)
            .to(to.parse()?)
            .subject(RESET_SUBJECT)
            .reply_to(REPLY_TO.parse()?)
            .header(ContentType::TEXT_HTML)
            .body(body)?;
        let creds = Credentials::new(SMTP_FROM.to_owned(), smtp_password.to_owned());
        let mailer = SmtpTransport::relay("smtp.gmail.com")?
            .credentials(creds)
            .build();
        // Send the email
        match mailer.send(&email) {
            Ok(result) => {
                tracing::info!("Email sent: {:?}", result);
                Ok(())
            }
            Err(e) => Err(e.into()),
        }
    }
}
