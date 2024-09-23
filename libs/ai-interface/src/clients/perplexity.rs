use crate::ai_constants::{OPENAI_VAR_NAME, PERPLEXITY_VAR_NAME};
use crate::clients::bulk::Role as SuperRole;
use crate::clients::bulk::{ChatCompletionExt, Message};
use crate::clients::openai::OpenAiClient;
use anyhow::{anyhow, Context};
use async_trait::async_trait;
use dotenv::dotenv;
use reqwest::header::{HeaderName, AUTHORIZATION};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env::VarError;
use std::fmt::{Display, Formatter};

pub struct PerplexityClient {
    client: Client,
    api_key: String,
}

impl PerplexityClient {
    pub fn new(client: Client, api_key: String) -> Self {
        Self { client, api_key }
    }

    pub fn from_env(client: Client, env_var_name: &str) -> Result<Self, VarError> {
        let api_key = std::env::var(env_var_name)?;
        Ok(Self::new(client, api_key))
    }
    async fn chat_completion(
        &self,
        request: &ChatCompletionRequest,
    ) -> anyhow::Result<ChatCompletionResponse> {
        let url = "https://api.perplexity.ai/chat/completions";
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

#[derive(Deserialize, Debug)]
struct InnerError {
    message: String,
    #[serde(rename = "type")]
    kind: String,
    param: Option<String>,
    code: String,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatCompletionRequest {
    model: String,
    messages: Vec<ChatMessage>,
}

impl ChatCompletionRequest {
    fn new(model: String, messages: Vec<ChatMessage>) -> Self {
        Self { model, messages }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    message: ChatMessage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatCompletionResponse {
    choices: Vec<Choice>,
}

#[async_trait]
impl ChatCompletionExt for PerplexityClient {
    async fn completion(&self, messages: Vec<Message>) -> anyhow::Result<Message> {
        let request = ChatCompletionRequest::new(
            String::from("llama-3.1-sonar-small-128k-online"),
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

#[ignore = "Needs valid Perplexity token"]
#[tokio::test]
async fn perplexity() {
    dotenv().ok();
    let client = PerplexityClient::from_env(Client::new(), PERPLEXITY_VAR_NAME).unwrap();
    let resp = client
        .chat_completion(&ChatCompletionRequest::new(
            String::from("llama-3.1-sonar-small-128k-online"),
            vec![ChatMessage::user(String::from("Introduce yourself"))],
        ))
        .await
        .unwrap();
    println!("resp = {:#?}", resp);
}
