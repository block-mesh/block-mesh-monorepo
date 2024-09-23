use crate::ai_constants::LLAMA_VAR_NAME;
use crate::clients::bulk::Role as SuperRole;
use crate::clients::bulk::{ChatCompletionExt, Message};
use anyhow::{anyhow, Context};
use async_trait::async_trait;
use dotenv::dotenv;
use reqwest::header::AUTHORIZATION;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env::VarError;

#[async_trait]
impl ChatCompletionExt for LlamaClient {
    async fn completion(&self, messages: Vec<Message>) -> anyhow::Result<Message> {
        let request = ChatRequest::new(
            String::from("llama3.1-405b"),
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
        let mut result = self.chat_completion(&request).await?;
        let choice = result
            .choices
            .pop()
            .context("Llama returned no completion messages")?;
        let role = match choice.message.role {
            Role::User => SuperRole::User,
            Role::Assistant => SuperRole::Assistant,
        };
        let content = choice.message.content;
        Ok(Message { content, role })
    }
}
pub struct LlamaClient {
    client: Client,
    api_key: String,
}

impl LlamaClient {
    pub fn new(client: Client, api_key: String) -> Self {
        Self { client, api_key }
    }
    pub fn from_env(client: Client, env_var_name: &str) -> Result<Self, VarError> {
        let api_key = std::env::var(env_var_name)?;
        Ok(Self::new(client, api_key))
    }

    async fn chat_completion(&self, chat_request: &ChatRequest) -> anyhow::Result<ChatResponse> {
        let url = "https://api.llama-api.com/chat/completions";
        let response = self
            .client
            .post(url)
            .header(AUTHORIZATION, format!("Bearer {}", self.api_key.as_str()))
            .json(&chat_request)
            .send()
            .await?;
        if response.status().is_success() {
            return Ok(response.json().await?);
        }
        if response.status().is_client_error() {
            let error: Value = response.json().await?;
            return Err(anyhow!(error));
        }
        Err(anyhow!(
            "Unexpected response status code {} for Meta Llama chat completion request",
            response.status()
        ))
    }
}

#[derive(Serialize, Debug)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    // functions: Vec<Function>,
    stream: bool,
    function_call: String,
}

impl ChatRequest {
    fn new(model: String, messages: Vec<ChatMessage>) -> Self {
        Self {
            model,
            messages,
            stream: false,
            function_call: String::from("none"),
        }
    }
}

#[derive(Deserialize, Debug)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    index: u32,
    message: ChatMessage,
    finish_reason: String,
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
#[serde(rename = "lowercase")]
enum Role {
    User,
    Assistant,
}

#[ignore = "Needs valid Meta Llama token"]
#[tokio::test]
async fn meta() {
    dotenv().ok();
    let client = LlamaClient::from_env(Client::new(), LLAMA_VAR_NAME).unwrap();
    let result = client
        .chat_completion(&ChatRequest::new(
            String::from("llama3.1-405b"),
            vec![ChatMessage::user(String::from("Introduce yourself"))],
        ))
        .await
        .unwrap();
}
