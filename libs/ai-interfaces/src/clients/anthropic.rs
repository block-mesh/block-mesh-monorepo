use crate::ai_constants::ANTHROPIC_VAR_NAME;
use crate::clients::bulk::Role as SuperRole;
use crate::clients::bulk::{ChatCompletionExt, Message};
use crate::models::anthropic::AnthropicModels;
use crate::models::base::ModelName;
use anyhow::{anyhow, Context};
use async_trait::async_trait;
use dotenv::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize, Serializer};
use std::env::VarError;
use std::fmt::{Display, Formatter};

#[async_trait]
impl ChatCompletionExt for AnthropicClient {
    async fn completion(
        &self,
        model_name: ModelName,
        messages: Vec<Message>,
    ) -> anyhow::Result<Message> {
        let request = ChatRequest {
            model: model_name.to_string(),
            max_tokens: 1024,
            messages: messages
                .into_iter()
                .map(|msg| {
                    if matches!(msg.role, SuperRole::User) {
                        ChatMessage::user(msg.content)
                    } else {
                        ChatMessage::assistant(msg.content)
                    }
                })
                .collect(),
        };
        let mut result = self.chat_completion(&request).await?;
        let role = match result.role {
            Role::User => SuperRole::User,
            Role::Assistant => SuperRole::Assistant,
        };
        let content = result
            .content
            .pop()
            .context("Anthropic returned no completion message")?
            .text;
        Ok(Message { role, content })
    }
}
pub struct AnthropicClient {
    client: Client,
    api_key: String,
}

impl AnthropicClient {
    pub fn new(client: Client, api_key: String) -> Self {
        Self { client, api_key }
    }
    pub fn from_env(client: Client, env_var_name: &str) -> Result<Self, VarError> {
        let api_key = std::env::var(env_var_name)?;
        Ok(Self::new(client, api_key))
    }

    async fn chat_completion(&self, chat_request: &ChatRequest) -> anyhow::Result<ChatResponse> {
        let url = "https://api.anthropic.com/v1/messages";
        let response = self
            .client
            .post(url)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(chat_request)
            .send()
            .await?;
        if response.status().is_success() {
            return Ok(response.json().await?);
        }
        if response.status().is_client_error() {
            let error: Error = response.json().await?;
            return Err(anyhow!(error));
        }
        Err(anyhow!(
            "Unexpected response status code {} for Anthorpic chat completion request",
            response.status()
        ))
    }
}

#[derive(Deserialize, Debug)]
struct Error {
    #[serde(rename = "type")]
    kind: String,
    error: InnerError,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.error)
    }
}

#[derive(Deserialize, Debug)]
struct InnerError {
    #[serde(rename = "type")]
    kind: String,
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum Role {
    User,
    Assistant,
}

#[derive(Serialize, Debug)]
struct ChatRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<ChatMessage>,
}

#[derive(Deserialize, Debug)]
struct ChatResponse {
    content: Vec<Content>,
    id: String,
    model: String,
    role: Role,
    stop_reason: String,
    stop_sequence: Option<String>,
    #[serde(rename = "type")]
    response_type: String,
    usage: Usage,
}

#[derive(Deserialize, Debug)]
struct Content {
    text: String,
    #[serde(rename = "type")]
    kind: String,
}

#[derive(Deserialize, Debug)]
struct Usage {
    input_tokens: u32,
    output_tokens: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatMessage {
    role: Role,
    content: String,
}

impl ChatMessage {
    fn user(content: String) -> Self {
        Self {
            role: Role::User,
            content,
        }
    }

    fn assistant(content: String) -> Self {
        Self {
            role: Role::Assistant,
            content,
        }
    }
}

#[ignore = "Needs valid Anthropic token"]
#[tokio::test]
async fn anthropic() {
    dotenv().ok();
    let client = AnthropicClient::from_env(Client::new(), ANTHROPIC_VAR_NAME).unwrap();
    let result = client
        .chat_completion(&ChatRequest {
            model: ModelName::Anthropic(AnthropicModels::default()).to_string(),
            max_tokens: 1024,
            messages: vec![ChatMessage::user(String::from("Introduce yourself"))],
        })
        .await
        .unwrap();
}
