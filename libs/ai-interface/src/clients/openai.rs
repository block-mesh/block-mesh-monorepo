use crate::ai_constants::OPENAI_VAR_NAME;
use crate::clients::bulk::Role as SuperRole;
use crate::clients::bulk::{ChatCompletionExt, Message};
use anyhow::{anyhow, Context};
use async_trait::async_trait;
use dotenv::dotenv;
use reqwest::header::AUTHORIZATION;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env::VarError;
use std::fmt::{Display, Formatter};

#[async_trait]
impl ChatCompletionExt for OpenAiClient {
    async fn completion(&self, messages: Vec<Message>) -> anyhow::Result<Message> {
        let request = ChatRequest::new(
            String::from("gpt-4o"),
            messages
                .into_iter()
                .map(|msg| {
                    if matches!(msg.role, SuperRole::User) {
                        ChatMessage::user(msg.content)
                    } else {
                        ChatMessage::assistant(msg.content)
                    }
                })
                .collect(),
        );
        let mut response = self.chat_completion(&request).await?;
        let message = response
            .choices
            .pop()
            .context("GPT returned no completion message")?;
        let content = message.message.content;
        let role = match message.message.role {
            Role::User => SuperRole::User,
            Role::Assistant => SuperRole::Assistant,
            other => return Err(anyhow!("Unimplemented GPT role {other}")),
        };
        Ok(Message { content, role })
    }
}
pub struct OpenAiClient {
    client: Client,
    api_key: String,
}

impl OpenAiClient {
    pub fn new(client: Client, api_key: String) -> Self {
        Self { client, api_key }
    }

    pub fn from_env(client: Client, env_var_name: &str) -> Result<Self, VarError> {
        let api_key = std::env::var(env_var_name)?;
        Ok(Self::new(client, api_key))
    }

    async fn chat_completion(&self, request: &ChatRequest) -> anyhow::Result<ChatResponse> {
        let url = "https://api.openai.com/v1/chat/completions";
        let response = self
            .client
            .post(url)
            .header(AUTHORIZATION, format!("Bearer {}", self.api_key.as_str()))
            .json(&request)
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
            "Unexpected response status code {} for OpenAI chat completion request",
            response.status()
        ))
    }
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
}

impl ChatRequest {
    fn new(model: String, messages: Vec<ChatMessage>) -> Self {
        Self { model, messages }
    }
}

#[derive(Deserialize)]
struct ChatResponse {
    pub(crate) choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    pub(crate) message: ChatMessage,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum Role {
    User,
    Assistant,
    System,
    Function,
}

impl Display for Role {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatMessage {
    pub(crate) role: Role,
    pub(crate) content: String,
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

#[derive(Deserialize, Debug)]
struct Error {
    error: InnerError,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.error)
    }
}

#[derive(Deserialize, Debug)]
struct InnerError {
    message: String,
    #[serde(rename = "type")]
    kind: String,
    param: Option<String>,
    code: String,
}

struct Metadata {}
enum RateLimitHeader {
    LimitRequests,
    LimitTokens,
    RemainingRequests,
    RemainingTokens,
    ResetRequests,
    ResetTokens,
}

impl Display for RateLimitHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "x-ratelimit-{}",
            match self {
                RateLimitHeader::LimitRequests => "limit-requests",
                RateLimitHeader::LimitTokens => "limit-tokens",
                RateLimitHeader::RemainingRequests => "remaining-requests",
                RateLimitHeader::RemainingTokens => "remaining-tokens",
                RateLimitHeader::ResetRequests => "reset-requests",
                RateLimitHeader::ResetTokens => "reset-tokens",
            }
        )
    }
}

#[ignore = "Needs valid OpenAI token"]
#[tokio::test]
async fn openai() {
    dotenv().ok();
    let client = OpenAiClient::from_env(Client::new(), OPENAI_VAR_NAME).unwrap();
    client
        .chat_completion(&ChatRequest::new(
            String::from("gpt-4o"),
            vec![ChatMessage::user(String::from("Introduce yourself"))],
        ))
        .await
        .unwrap();
}
