use aws_config::meta::region::RegionProviderChain;
use aws_sdk_sesv2::config::Region;
use reqwest::{Client, ClientBuilder};
use std::time::Duration;

pub const _BASE_URL: &str = "https://api.mailgun.net/v3/blockmesh.xyz/messages";
pub const _CONFIRM_TEMPLATE_ID: &str = "confirmation email";
pub const _RESET_TEMPLATE_ID: &str = "reset password";
pub const EMAIL: &str = "support@perceptrons.xyz";
pub const SMTP_FROM: &str = "support@perceptrons.xyz";
pub const _SUBJECT: &str = "Perceptron Network";
pub const REPLY_TO: &str = "support@perceptrons.xyz";
pub const CONFIRM_SUBJECT: &str = "Confirmation Email from Perceptron Network";
pub const RESET_SUBJECT: &str = "Reset Password from Perceptron Network";

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
}
