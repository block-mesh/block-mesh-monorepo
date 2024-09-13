#![allow(unused)]

use anyhow::anyhow;
use reqwest::header::AUTHORIZATION;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env::VarError;
use std::fmt::{Display, Formatter};

const ENV_VAR_NAME: &str = "OPENAI_API_KEY";
struct OpenAiClient {
    client: Client,
    api_key: String,
}

impl OpenAiClient {
    fn new(client: Client, api_key: String) -> Self {
        Self { client, api_key }
    }

    fn from_env(client: Client, env_var_name: &str) -> Result<Self, VarError> {
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
            "Unexpected response status code {} for chat completion request",
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
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ChatMessage,
}

#[derive(Serialize, Deserialize, Debug)]
enum Role {
    User,
    Assistant,
    System,
    Function,
}

impl Display for Role {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Role::User => "user",
                Role::Assistant => "assistant",
                Role::System => "system",
                Role::Function => "function",
            }
        )
    }
}

impl TryFrom<String> for Role {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(match value.as_str() {
            "user" => Self::User,
            "assistant" => Self::Assistant,
            "system" => Self::System,
            "function" => Self::Function,
            other => return Err(format!("Unrecognised role '{other}'")),
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatMessage {
    role: Role,
    content: String,
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
