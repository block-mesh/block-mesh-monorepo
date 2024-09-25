use crate::ai_constants::{
    ANTHROPIC_VAR_NAME, GEMINI_VAR_NAME, LLAMA_VAR_NAME, MISTRAL_VAR_NAME, OPENAI_VAR_NAME,
    PERPLEXITY_VAR_NAME,
};
use crate::clients::anthropic::AnthropicClient;
use crate::clients::google::GeminiClient;
use crate::clients::meta::LlamaClient;
use crate::clients::mistral::MistralClient;
use crate::clients::openai::OpenAiClient;
use crate::clients::perplexity::PerplexityClient;
use crate::questions::generate_questions;
use async_trait::async_trait;
use dotenv::dotenv;
use futures::future::join_all;
use futures::task::SpawnExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::env::VarError;
use std::fmt::Display;
use std::fs::File;
use std::io::Write;
use tokio::task::JoinSet;

pub struct AIClient {
    anthropic: AnthropicClient,
    google: GeminiClient,
    meta: LlamaClient,
    mistral: MistralClient,
    openai: OpenAiClient,
    perplexity: PerplexityClient,
}

#[derive(Hash, PartialEq, Eq, Serialize, Deserialize, Clone, Debug)]
pub enum ClientKind {
    Anthropic,
    Google,
    Meta,
    Mistral,
    OpenAi,
    Perplexity,
}

impl AIClient {
    pub fn new() -> Self {
        let client = reqwest::Client::new();
        let anthropic = AnthropicClient::from_env(client.clone(), ANTHROPIC_VAR_NAME).unwrap();
        let google = GeminiClient::from_env(client.clone(), GEMINI_VAR_NAME).unwrap();
        let meta = LlamaClient::from_env(client.clone(), LLAMA_VAR_NAME).unwrap();
        let mistral = MistralClient::from_env(client.clone(), MISTRAL_VAR_NAME).unwrap();
        let openai = OpenAiClient::from_env(client.clone(), OPENAI_VAR_NAME).unwrap();
        let perplexity = PerplexityClient::from_env(client, PERPLEXITY_VAR_NAME).unwrap();
        Self {
            anthropic,
            google,
            meta,
            mistral,
            openai,
            perplexity,
        }
    }
    pub async fn completions(
        &self,
        client_kinds: impl Into<HashSet<ClientKind>>,
        messages: Vec<Message>,
    ) -> AIClientResponses {
        let mut responses = AIClientResponses::default();
        let mut set = JoinSet::new();
        for kind in client_kinds.into().into_iter() {
            let messages = messages.clone();
            let future = match kind {
                ClientKind::Perplexity => self
                    .perplexity
                    .clone()
                    .completion(messages, String::from("llama-3.1-sonar-small-128k-online")),
                ClientKind::Anthropic => self
                    .anthropic
                    .clone()
                    .completion(messages, crate::clients::anthropic::Model::Sonnet),
                ClientKind::Google => self
                    .google
                    .clone()
                    .completion(messages, String::from("gemini-1.5-flash-latest")),
                ClientKind::Meta => self
                    .meta
                    .clone()
                    .completion(messages, String::from("llama3.1-405b")),
                ClientKind::Mistral => self
                    .mistral
                    .clone()
                    .completion(messages, String::from("mistral-small-latest")),
                ClientKind::OpenAi => self
                    .openai
                    .clone()
                    .completion(messages, String::from("gpt-4o")),
            };
            set.spawn(async move { (future.await, kind) });
        }
        while let Some(Ok((response, client_kind))) = set.join_next().await {
            responses.insert(client_kind, Some(response));
        }
        println!("Question: {:?}, responses: {:?}", messages, responses);
        responses
    }
}

pub type AIClientResponses = HashMap<ClientKind, Option<anyhow::Result<Message>>>;

#[derive(Debug, Clone)]
pub enum Role {
    User,
    Assistant,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub content: String,
    pub role: Role,
}

#[async_trait]
pub trait ChatCompletionExt {
    type Model: Display;
    async fn completion(
        self,
        messages: Vec<Message>,
        model: Self::Model,
    ) -> anyhow::Result<Message>;
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct Response {
    model: ClientKind,
    question: String,
    answer: String,
}

#[ignore = "Requires all AI API keys"]
#[tokio::test]
async fn bulk_message_propagation() {
    dotenv().ok();
    let client = AIClient::new();

    let root = std::env::var("ROOT").unwrap();
    let now = chrono::offset::Utc::now();
    let dir_name = format!("{}/ai-results/", root);
    let file_name = format!("{}/ai-results/{}.json", root, now.timestamp_millis());
    std::fs::create_dir_all(dir_name).unwrap();

    let questions = generate_questions();
    let mut final_responses: Vec<Response> = Vec::new();

    for question in questions {
        let responses = client
            .completions(
                [
                    // ClientKind::Anthropic,
                    // ClientKind::Google,
                    // ClientKind::Meta,
                    // ClientKind::Mistral,
                    // ClientKind::OpenAi,
                    ClientKind::Perplexity,
                ],
                vec![question.clone()],
            )
            .await;
        responses.iter().for_each(|response| {
            let q = question.clone();
            let model = response.0;
            if let Some(r1) = response.1 {
                if let Ok(r2) = r1 {
                    final_responses.push(Response {
                        question: q.content,
                        model: model.clone(),
                        answer: r2.clone().content,
                    })
                }
            }
        });
    }

    let json = serde_json::to_string_pretty(&final_responses).unwrap();
    let mut file = File::create(file_name).unwrap();
    file.write_all(json.as_ref()).unwrap();
}
