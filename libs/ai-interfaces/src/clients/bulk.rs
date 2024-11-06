use crate::ai_constants::{
    ANTHROPIC_VAR_NAME, GEMINI_VAR_NAME, LLAMA_VAR_NAME, MISTRAL_VAR_NAME, OPENAI_VAR_NAME,
    PERPLEXITY_VAR_NAME,
};
use crate::clients::anthropic::AnthropicClient;
use crate::clients::bulk::ClientKind::Perplexity;
use crate::clients::google::GeminiClient;
use crate::clients::meta::LlamaClient;
use crate::clients::mistral::MistralClient;
use crate::clients::openai::OpenAiClient;
use crate::clients::perplexity::PerplexityClient;
use crate::models::anthropic::AnthropicModels;
use crate::models::base::ModelName;
use crate::models::google::GoogleModels;
use crate::models::meta::MetaModels;
use crate::models::mistral::MistralModels;
use crate::models::open_ai::OpenAiModels;
use crate::models::perplexity::PerplexityModels;
use async_trait::async_trait;
use dotenv::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::env::VarError;
use std::fs::File;
use std::io::Write;

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
    pub fn new() -> Result<Self, VarError> {
        let client = reqwest::Client::new();
        let anthropic = AnthropicClient::from_env(client.clone(), ANTHROPIC_VAR_NAME)?;
        let google = GeminiClient::from_env(client.clone(), GEMINI_VAR_NAME)?;
        let meta = LlamaClient::from_env(client.clone(), LLAMA_VAR_NAME)?;
        let mistral = MistralClient::from_env(client.clone(), MISTRAL_VAR_NAME)?;
        let openai = OpenAiClient::from_env(client.clone(), OPENAI_VAR_NAME)?;
        let perplexity = PerplexityClient::from_env(client, PERPLEXITY_VAR_NAME)?;
        Ok(Self {
            anthropic,
            google,
            meta,
            mistral,
            openai,
            perplexity,
        })
    }
    pub async fn completions(
        &self,
        client_kinds: impl Into<HashSet<ClientKind>>,
        messages: Vec<Message>,
    ) -> AIClientResponses {
        let mut responses = AIClientResponses::default();
        for kind in client_kinds.into().into_iter() {
            match kind {
                ClientKind::Perplexity => {
                    responses.insert(
                        ClientKind::Perplexity,
                        Some(
                            self.perplexity
                                .completion(
                                    ModelName::Perplexity(PerplexityModels::default()),
                                    messages.clone(),
                                )
                                .await,
                        ),
                    );
                }
                ClientKind::Anthropic => {
                    responses.insert(
                        ClientKind::Anthropic,
                        Some(
                            self.anthropic
                                .completion(
                                    ModelName::Anthropic(AnthropicModels::default()),
                                    messages.clone(),
                                )
                                .await,
                        ),
                    );
                }
                ClientKind::Google => {
                    responses.insert(
                        ClientKind::Google,
                        Some(
                            self.google
                                .completion(
                                    ModelName::Google(GoogleModels::default()),
                                    messages.clone(),
                                )
                                .await,
                        ),
                    );
                }
                ClientKind::Meta => {
                    responses.insert(
                        ClientKind::Meta,
                        Some(
                            self.meta
                                .completion(
                                    ModelName::Meta(MetaModels::default()),
                                    messages.clone(),
                                )
                                .await,
                        ),
                    );
                }
                ClientKind::Mistral => {
                    responses.insert(
                        ClientKind::Mistral,
                        Some(
                            self.mistral
                                .completion(
                                    ModelName::Mistral(MistralModels::default()),
                                    messages.clone(),
                                )
                                .await,
                        ),
                    );
                }
                ClientKind::OpenAi => {
                    responses.insert(
                        ClientKind::OpenAi,
                        Some(
                            self.openai
                                .completion(
                                    ModelName::OpenAi(OpenAiModels::default()),
                                    messages.clone(),
                                )
                                .await,
                        ),
                    );
                }
            };
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
    async fn completion(
        &self,
        model_name: ModelName,
        messages: Vec<Message>,
    ) -> anyhow::Result<Message>;
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct Response {
    model: ClientKind,
    question: String,
    answer: String,
}
