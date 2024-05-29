use reqwest::{multipart, Client};
use secret::Secret;
use serde_json::json;

const BASE_URL: &str = "https://api.mailgun.net/v3/blockmesh.xyz/messages";
const CONFIRM_TEMPLATE_ID: &str = "confirmation email";
const EMAIL: &str = "no-reply@blockmesh.xyz";
const SUBJECT: &str = "BlockMesh Network";

pub struct EmailClient {
    pub client: Client,
    pub token: Secret<String>,
    pub base_url: String,
}

impl EmailClient {
    pub fn new(token: Secret<String>, base_url: String) -> Self {
        Self {
            base_url,
            client: Client::new(),
            token,
        }
    }
    #[tracing::instrument(name = "send_confirmation_email", skip(self, token))]
    pub async fn send_confirmation_email(&self, to: &str, token: &str) {
        let form = multipart::Form::new()
            .text("from", format!("BlockMesh Network<{}", EMAIL))
            .text("to", format!("<{}>", to))
            .text("subject", SUBJECT)
            .text("template", CONFIRM_TEMPLATE_ID)
            .text(
                "h:X-Mailgun-Variables",
                json!({"action_url": format!("{}/email_confirm?token={}", self.base_url, token)})
                    .to_string(),
            );
        let result = self
            .client
            .post(BASE_URL)
            .basic_auth("api", Some(self.token.expose_secret()))
            .header("Authorization", format!("Bearer {}", self.token))
            .multipart(form)
            .send()
            .await;
        tracing::info!("Email sent: {:?}", result);
    }
}
