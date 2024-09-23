use crate::ai_constants::MISTRAL_VAR_NAME;
use crate::clients::bulk::Role as SuperRole;
use crate::clients::bulk::{ChatCompletionExt, Message};
use anyhow::{anyhow, Context};
use async_trait::async_trait;
use dotenv::dotenv;
use reqwest::header::AUTHORIZATION;
use reqwest::Client;
use serde::__private::de::Content;
use serde::{Deserialize, Serialize};
use std::env::VarError;
use std::fmt::{Display, Formatter};

#[async_trait]
impl ChatCompletionExt for MistralClient {
    async fn completion(&self, messages: Vec<Message>) -> anyhow::Result<Message> {
        let request = ChatRequest::new(
            String::from("mistral-small-latest"),
            messages
                .into_iter()
                .map(|msg| {
                    if matches!(msg.role, SuperRole::User) {
                        ChatMessage::user(msg.content)
                    } else {
                        ChatMessage::assistant(msg.content, false)
                    }
                })
                .collect(),
        );
        let mut result = self.chat_completion(&request).await?;
        let message = result
            .choices
            .pop()
            .context("Mistral returned no completion messages")?
            .message;
        let content = message
            .content
            .context("Mistral should have included a non-empty string in the response")?;
        let role = match message.role {
            Role::User => SuperRole::User,
            Role::Assistant => SuperRole::Assistant,
        };
        Ok(Message { content, role })
    }
}
pub struct MistralClient {
    client: Client,
    api_key: String,
}

impl MistralClient {
    pub fn new(client: Client, api_key: String) -> Self {
        Self { client, api_key }
    }
    pub fn from_env(client: Client, env_var_name: &str) -> Result<Self, VarError> {
        let api_key = std::env::var(env_var_name)?;
        Ok(Self::new(client, api_key))
    }
    async fn chat_completion(&self, chat_request: &ChatRequest) -> anyhow::Result<ChatResponse> {
        let url = "https://api.mistral.ai/v1/chat/completions";
        let response = self
            .client
            .post(url)
            .header(AUTHORIZATION, format!("Bearer {}", self.api_key.as_str()))
            .json(chat_request)
            .send()
            .await?;
        if response.status().is_success() {
            return Ok(response.json().await?);
        }
        if response.status() == 422 {
            let error: Error = response.json().await?;
            return Err(anyhow!(error));
        }
        Err(anyhow!(
            "Unexpected response status code {} for Mistral chat completion request",
            response.status()
        ))
    }
}
#[derive(Deserialize, Debug)]
struct ChatResponse {
    id: String,
    object: String,
    model: String,
    usage: UsageInfo,
    created: u64,
    choices: Vec<ChatCompletionChoice>,
}

#[derive(Deserialize, Debug)]
struct ChatCompletionChoice {
    index: u64,
    message: ChatMessage,
    finish_reason: FinishReason,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
enum FinishReason {
    Stop,
    Length,
    ModelLength,
    Error,
    ToolCalls,
}

#[derive(Deserialize, Debug)]
struct UsageInfo {
    prompt_tokens: u64,
    completion_tokens: u64,
    total_tokens: u64,
}
#[derive(Deserialize, Debug)]
struct Error {
    detail: Vec<Detail>,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.detail)
    }
}

#[derive(Deserialize, Debug)]
struct Detail {
    loc: Vec<String>,
    msg: String,
    #[serde(rename = "type")]
    kind: String,
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

#[derive(Serialize, Deserialize, Debug)]
struct ChatMessage {
    role: Role,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<bool>,
}

impl ChatMessage {
    fn user(content: String) -> Self {
        Self {
            role: Role::User,
            content: Some(content),
            prefix: None,
        }
    }
    fn assistant(content: String, prefix: bool) -> Self {
        Self {
            role: Role::Assistant,
            content: Some(content),
            prefix: Some(prefix),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum Role {
    User,
    Assistant,
}

#[ignore = "Needs valid Mistral token"]
#[tokio::test]
async fn mistral() {
    dotenv().ok();
    let client = MistralClient::from_env(Client::new(), MISTRAL_VAR_NAME).unwrap();
    let request = ChatRequest::new(
        String::from("mistral-small-latest"),
        vec![ChatMessage::user(String::from("Introduce yourself"))],
    );
    let result = client.chat_completion(&request).await.unwrap();
    println!("{result:#?}")
}
