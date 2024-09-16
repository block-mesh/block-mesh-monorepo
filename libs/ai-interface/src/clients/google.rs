use crate::clients::{ChatCompletionExt, Message};
use anyhow::{anyhow, Context};
use async_trait::async_trait;
use dotenv::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env::VarError;

const ENV_VAR_NAME: &str = "GOOGLE_GEMINI_API_KEY";

#[async_trait]
impl ChatCompletionExt for GeminiClient {
    async fn completion(&self, messages: Vec<Message>) -> anyhow::Result<Message> {
        let request = ChatRequest::new(
            messages
                .into_iter()
                .map(|msg| {
                    if matches!(msg.role, super::Role::User) {
                        ChatMessage::user(vec![Part::Text(msg.content)])
                    } else {
                        ChatMessage::model(vec![Part::Text(msg.content)])
                    }
                })
                .collect(),
        );
        let mut result = self.chat_completion(&request).await?;
        let part = result
            .candidates
            .pop()
            .context("Gemini returned no completion candidates")?
            .content
            .pop()
            .context("Gemini returned no completion content")?
            .parts
            .pop()
            .context("Gemini returned no completion messages")?;
        let content = match part {
            Part::Text(text) => text,
            Part::InlineData { mime_type, .. } => {
                return Err(anyhow!(
                    "Unexpected MIME type '{mime_type}' in Gemini completion"
                ))
            }
        };
        let role = super::Role::User;
        Ok(Message { content, role })
    }
}
pub struct GeminiClient {
    client: Client,
    api_key: String,
}

impl GeminiClient {
    pub fn new(client: Client, api_key: String) -> Self {
        Self { client, api_key }
    }

    pub fn from_env(client: Client, env_var_name: &str) -> Result<Self, VarError> {
        let api_key = std::env::var(env_var_name)?;
        Ok(Self::new(client, api_key))
    }

    async fn chat_completion(&self, chat_request: &ChatRequest) -> anyhow::Result<ChatResponse> {
        let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-latest:generateContent?key={}", self.api_key);
        let response = self.client.post(url).json(chat_request).send().await?;
        if response.status().is_success() {
            return Ok(response.json().await?);
        }
        if response.status().is_client_error() {
            let error: Value = response.json().await?;
            return Err(anyhow!(error));
        }
        Err(anyhow!(
            "Unexpected response status code {} for Google Gemini chat completion request",
            response.status()
        ))
    }
}

#[derive(Serialize, Debug)]
struct ChatRequest {
    contents: Vec<ChatMessage>,
}

impl ChatRequest {
    fn new(messages: Vec<ChatMessage>) -> Self {
        Self { contents: messages }
    }
}
#[derive(Deserialize, Debug)]
struct ChatResponse {
    candidates: Vec<Candidate>,
    usage_metadata: UsageMetadata,
}

#[derive(Deserialize, Debug)]
struct Candidate {
    content: Vec<ChatMessage>,
    finish_reason: String,
    index: u32,
    safety_ratings: Vec<SafetyRating>,
}

#[derive(Deserialize, Debug)]
struct SafetyRating {
    category: String,
    probability: String,
}

#[derive(Deserialize, Debug)]
struct UsageMetadata {
    prompt_token_count: u32,
    candidates_token_count: u32,
    total_token_count: u32,
}
#[derive(Serialize, Deserialize, Debug)]
struct ChatMessage {
    role: Role,
    parts: Vec<Part>,
}

impl ChatMessage {
    fn user(parts: Vec<Part>) -> Self {
        Self {
            role: Role::User,
            parts,
        }
    }
    fn model(parts: Vec<Part>) -> Self {
        Self {
            role: Role::Model,
            parts,
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
enum Part {
    Text(String),
    InlineData { mime_type: String, data: String },
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "lowercase")]
enum Role {
    User,
    Model,
}

#[ignore = "Need valid Google Gemini token"]
#[tokio::test]
async fn google_gemini() {
    dotenv().ok();
    let client = GeminiClient::from_env(Client::new(), ENV_VAR_NAME).unwrap();
    let response = client
        .chat_completion(&ChatRequest::new(vec![ChatMessage::user(vec![
            Part::Text(String::from("Introduce yourself")),
        ])]))
        .await;
}
#[test]
fn test_parts_parsing() {
    let p1 = Part::Text(String::from("Some text"));
    let s = serde_json::to_string(&p1).unwrap();
    println!("{s}");
    let p2 = Part::InlineData {
        mime_type: String::from("image/jpeg"),
        data: String::from("base64 string"),
    };
    let s = serde_json::to_string(&p2).unwrap();
    println!("{s}");
}
