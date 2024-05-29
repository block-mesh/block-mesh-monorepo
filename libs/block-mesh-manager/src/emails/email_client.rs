use reqwest::Client;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
const BASE_URL: &str = "https://api.mailjet.com/v3.1/send";
pub struct EmailClient {
    pub client: Client,
    pub mj_apikey_public: String,
    pub mj_apikey_private: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase"))]
struct EmailAddress {
    email: String,
    name: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all(serialize = "PascalCase"))]
struct EmailMessage {
    from: EmailAddress,
    to: Vec<EmailAddress>,
    subject: String,
    text_part: String,
    #[serde(rename = "HTMLPart")]
    html_part: String,
}

impl EmailClient {
    #[allow(dead_code)]
    pub async fn send_email(&self, to: String, subject: String, text: String, html: String) {
        let messages: Vec<EmailMessage> = vec![EmailMessage {
            subject,
            from: EmailAddress {
                email: "no-reply@blockmesh.xyz".to_string(),
                name: "BlockMesh Network".to_string(),
            },
            to: vec![EmailAddress {
                email: to,
                name: "".to_string(),
            }],
            text_part: text,
            html_part: html,
        }];

        let result = self
            .client
            .post(BASE_URL)
            .basic_auth(&self.mj_apikey_public, Some(&self.mj_apikey_private))
            .json(&messages)
            .send()
            .await;
        tracing::info!("Email sent: {:?}", result);
    }
}
