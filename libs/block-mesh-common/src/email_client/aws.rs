use crate::email_client::client::{EmailClient, CONFIRM_SUBJECT, EMAIL, REPLY_TO, RESET_SUBJECT};
use crate::email_client::confirm_email::CONFIRM_EMAIL;
use crate::email_client::reset_email::RESET_EMAIL;
use aws_sdk_sesv2::operation::send_email::SendEmailOutput;
use aws_sdk_sesv2::types::{Body, Content, Destination, EmailContent, Message};

impl EmailClient {
    #[tracing::instrument(name = "send_confirmation_email_aws", skip_all, ret, err)]
    pub async fn send_confirmation_email_aws(
        &self,
        to: &str,
        token: &str,
    ) -> anyhow::Result<SendEmailOutput> {
        let mut dest: Destination = Destination::builder().build();
        dest.to_addresses = Some(vec![to.to_string()]);
        let subject_content = Content::builder()
            .data(CONFIRM_SUBJECT)
            .charset("UTF-8")
            .build()?;
        let body_content = Content::builder()
            .data(CONFIRM_EMAIL.replace(
                "{{action_url}}",
                &format!(
                    "{}/email_confirm?token={}&email={}",
                    self.base_url, token, to
                ),
            ))
            .charset("UTF-8")
            .build()?;
        let body = Body::builder().html(body_content).build();
        let msg = Message::builder()
            .subject(subject_content)
            .body(body)
            .build();
        let email_content = EmailContent::builder().simple(msg).build();
        Ok(self
            .aws_client
            .send_email()
            .from_email_address(EMAIL)
            .reply_to_addresses(REPLY_TO)
            .destination(dest)
            .content(email_content)
            .send()
            .await?)
    }

    #[tracing::instrument(name = "send_reset_password_email_aws", skip_all, ret, err)]
    pub async fn send_reset_password_email_aws(
        &self,
        to: &str,
        token: &str,
    ) -> anyhow::Result<SendEmailOutput> {
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
        Ok(self
            .aws_client
            .send_email()
            .from_email_address(EMAIL)
            .reply_to_addresses(REPLY_TO)
            .destination(dest)
            .content(email_content)
            .send()
            .await?)
    }
}
