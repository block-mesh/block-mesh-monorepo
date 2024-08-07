use crate::emails::confirm_email::CONFIRM_EMAIL;
use crate::emails::reset_email::RESET_EMAIL;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_sesv2::config::Region;
use aws_sdk_sesv2::types::{Body, Content, Destination, EmailContent, Message};
use reqwest::Client;

const _BASE_URL: &str = "https://api.mailgun.net/v3/blockmesh.xyz/messages";
const _CONFIRM_TEMPLATE_ID: &str = "confirmation email";
const _RESET_TEMPLATE_ID: &str = "reset password";
const EMAIL: &str = "no-reply@blockmesh.xyz";
const SUBJECT: &str = "BlockMesh Network";

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
            client: Client::new(),
            aws_client,
        }
    }
    #[tracing::instrument(name = "send_confirmation_email", skip(self, token))]
    pub async fn send_confirmation_email(&self, to: &str, token: &str) -> anyhow::Result<()> {
        let mut dest: Destination = Destination::builder().build();
        dest.to_addresses = Some(vec![to.to_string()]);
        let subject_content = Content::builder().data(SUBJECT).charset("UTF-8").build()?;
        let body_content = Content::builder()
            .data(CONFIRM_EMAIL.replace(
                "{{action_url}}",
                &format!("{}/email_confirm={}", self.base_url, token),
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
            .destination(dest)
            .content(email_content)
            .send()
            .await?;
        tracing::info!("Email sent: {:?}", result);
        Ok(())
    }

    #[tracing::instrument(name = "send_reset_password_email", skip(self, token))]
    pub async fn send_reset_password_email(&self, to: &str, token: &str) -> anyhow::Result<()> {
        let mut dest: Destination = Destination::builder().build();
        dest.to_addresses = Some(vec![to.to_string()]);
        let subject_content = Content::builder().data(SUBJECT).charset("UTF-8").build()?;
        let body_content = Content::builder()
            .data(RESET_EMAIL.replace(
                "{{action_url}}",
                &format!("{}/new_password={}", self.base_url, token),
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
            .destination(dest)
            .content(email_content)
            .send()
            .await?;
        tracing::info!("Email sent: {:?}", result);
        Ok(())
    }
}
