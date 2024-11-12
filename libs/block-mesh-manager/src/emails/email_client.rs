use crate::emails::confirm_email::CONFIRM_EMAIL;
use crate::emails::reset_email::RESET_EMAIL;
use crate::utils::cache_envar::get_envar;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_sesv2::config::Region;
use aws_sdk_sesv2::types::{Body, Content, Destination, EmailContent, Message};
use block_mesh_common::env::app_env_var::AppEnvVar;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message as LetterMessage, SmtpTransport, Transport};
use reqwest::{Client, ClientBuilder};
use std::time::Duration;

const _BASE_URL: &str = "https://api.mailgun.net/v3/blockmesh.xyz/messages";
const _CONFIRM_TEMPLATE_ID: &str = "confirmation email";
const _RESET_TEMPLATE_ID: &str = "reset password";
const EMAIL: &str = "support@blockmesh.xyz";
const GMAIL_FROM: &str = "support@blockmesh.xyz";
const _SUBJECT: &str = "BlockMesh Network";
const REPLY_TO: &str = "support@blockmesh.xyz";

const CONFIRM_SUBJECT: &str = "Confirmation Email from BlockMesh Network";
const RESET_SUBJECT: &str = "Reset Password from BlockMesh Network";

pub struct EmailClient {
    pub client: Client,
    pub base_url: String,
    pub aws_client: aws_sdk_sesv2::Client,
}

impl EmailClient {
    pub async fn new(base_url: String) -> Self {
        let region_provider = RegionProviderChain::first_try(Region::from_static("eu-north-1"))
            .or_default_provider()
            .or_else(Region::new("us-west-2"));
        let shared_config = aws_config::from_env().region(region_provider).load().await;
        let aws_client = aws_sdk_sesv2::Client::new(&shared_config);

        Self {
            base_url,
            client: ClientBuilder::new()
                .timeout(Duration::from_secs(3))
                .build()
                .unwrap_or_default(),
            aws_client,
        }
    }
    #[tracing::instrument(name = "send_confirmation_email_aws", skip_all, ret, err)]
    pub async fn send_confirmation_email_aws(&self, to: &str, token: &str) -> anyhow::Result<()> {
        let mut dest: Destination = Destination::builder().build();
        dest.to_addresses = Some(vec![to.to_string()]);
        let subject_content = Content::builder()
            .data(CONFIRM_SUBJECT)
            .charset("UTF-8")
            .build()?;
        let body_content = Content::builder()
            .data(CONFIRM_EMAIL.replace(
                "{{action_url}}",
                &format!("{}/email_confirm?token={}", self.base_url, token),
            ))
            .charset("UTF-8")
            .build()?;
        let body = Body::builder().html(body_content).build();
        let msg = Message::builder()
            .subject(subject_content)
            .body(body)
            .build();
        let email_content = EmailContent::builder().simple(msg).build();
        let result = self
            .aws_client
            .send_email()
            .from_email_address(EMAIL)
            .reply_to_addresses(REPLY_TO)
            .destination(dest)
            .content(email_content)
            .send()
            .await?;
        tracing::info!("Email sent: {:?}", result);
        Ok(())
    }

    #[tracing::instrument(name = "send_confirmation_email_gmail", skip_all, ret, err)]
    pub async fn send_confirmation_email_gmail(&self, to: &str, token: &str) -> anyhow::Result<()> {
        let gmail_password = get_envar(&AppEnvVar::GmailAppPassword.to_string()).await;
        let body = CONFIRM_EMAIL.replace(
            "{{action_url}}",
            &format!("{}/email_confirm?token={}", self.base_url, token),
        );
        let email = LetterMessage::builder()
            .from(GMAIL_FROM.parse()?)
            .to(to.parse()?)
            .subject(CONFIRM_SUBJECT)
            .reply_to(REPLY_TO.parse()?)
            .header(ContentType::TEXT_HTML)
            .body(body)?;
        let creds = Credentials::new(GMAIL_FROM.to_owned(), gmail_password.to_owned());
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

    #[tracing::instrument(name = "send_reset_password_email_aws", skip_all, ret, err)]
    pub async fn send_reset_password_email_aws(&self, to: &str, token: &str) -> anyhow::Result<()> {
        let mut dest: Destination = Destination::builder().build();
        dest.to_addresses = Some(vec![to.to_string()]);
        let subject_content = Content::builder()
            .data(RESET_SUBJECT)
            .charset("UTF-8")
            .build()?;
        let body_content = Content::builder()
            .data(RESET_EMAIL.replace(
                "{{action_url}}",
                &format!("{}/new_password?token={}", self.base_url, token),
            ))
            .charset("UTF-8")
            .build()?;
        let body = Body::builder().html(body_content).build();
        let msg = Message::builder()
            .subject(subject_content)
            .body(body)
            .build();
        let email_content = EmailContent::builder().simple(msg).build();
        let result = self
            .aws_client
            .send_email()
            .from_email_address(EMAIL)
            .reply_to_addresses(REPLY_TO)
            .destination(dest)
            .content(email_content)
            .send()
            .await?;
        tracing::info!("Email sent: {:?}", result);
        Ok(())
    }

    #[tracing::instrument(name = "send_reset_password_email_gmail", skip_all, ret, err)]
    pub async fn send_reset_password_email_gmail(
        &self,
        to: &str,
        token: &str,
    ) -> anyhow::Result<()> {
        let gmail_password = get_envar(&AppEnvVar::GmailAppPassword.to_string()).await;
        let body = RESET_EMAIL.replace(
            "{{action_url}}",
            &format!("{}/new_password?token={}", self.base_url, token),
        );
        let email = LetterMessage::builder()
            .from(GMAIL_FROM.parse()?)
            .to(to.parse()?)
            .subject(RESET_SUBJECT)
            .reply_to(REPLY_TO.parse()?)
            .header(ContentType::TEXT_HTML)
            .body(body)?;
        let creds = Credentials::new(GMAIL_FROM.to_owned(), gmail_password.to_owned());
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
