use crate::clients::anthropic::AnthropicClient;
use crate::clients::google::GeminiClient;
use crate::clients::meta::LlamaClient;
use crate::clients::mistral::MistralClient;
use crate::clients::openai::OpenAiClient;
use async_trait::async_trait;
use dotenv::dotenv;
use reqwest::Client;
use std::collections::HashSet;
use std::env::VarError;

pub mod anthropic;
pub mod google;
pub mod meta;
pub mod mistral;
pub mod openai;

const ANTHROPIC_VAR_NAME: &str = "ANTHROPIC_API_KEY";
const GEMINI_VAR_NAME: &str = "GOOGLE_GEMINI_API_KEY";
const LLAMA_VAR_NAME: &str = "META_LLAMA_API_KEY";
const MISTRAL_VAR_NAME: &str = "MISTRAL_API_KEY";
const OPENAI_VAR_NAME: &str = "OPENAI_API_KEY";

pub struct AIClient {
    anthropic: AnthropicClient,
    google: GeminiClient,
    meta: LlamaClient,
    mistral: MistralClient,
    openai: OpenAiClient,
}

#[derive(Hash, PartialEq, Eq)]
pub enum ClientKind {
    Anthropic,
    Google,
    Meta,
    Mistral,
    OpenAi,
}

impl AIClient {
    pub fn new() -> Result<Self, VarError> {
        let client = reqwest::Client::new();
        let anthropic = AnthropicClient::from_env(client.clone(), ANTHROPIC_VAR_NAME)?;
        let google = GeminiClient::from_env(client.clone(), GEMINI_VAR_NAME)?;
        let meta = LlamaClient::from_env(client.clone(), LLAMA_VAR_NAME)?;
        let mistral = MistralClient::from_env(client.clone(), MISTRAL_VAR_NAME)?;
        let openai = OpenAiClient::from_env(client, OPENAI_VAR_NAME)?;
        Ok(Self {
            anthropic,
            google,
            meta,
            mistral,
            openai,
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
                ClientKind::Anthropic => {
                    responses.anthropic = Some(self.anthropic.completion(messages.clone()).await);
                }
                ClientKind::Google => {
                    responses.google = Some(self.google.completion(messages.clone()).await)
                }
                ClientKind::Meta => {
                    responses.meta = Some(self.meta.completion(messages.clone()).await)
                }
                ClientKind::Mistral => {
                    responses.mistral = Some(self.mistral.completion(messages.clone()).await)
                }
                ClientKind::OpenAi => {
                    responses.openai = Some(self.openai.completion(messages.clone()).await)
                }
            };
        }
        responses
    }
}

#[derive(Debug, Default)]
pub struct AIClientResponses {
    pub anthropic: Option<anyhow::Result<Message>>,
    pub google: Option<anyhow::Result<Message>>,
    pub meta: Option<anyhow::Result<Message>>,
    pub mistral: Option<anyhow::Result<Message>>,
    pub openai: Option<anyhow::Result<Message>>,
}

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
    async fn completion(&self, messages: Vec<Message>) -> anyhow::Result<Message>;
}

#[ignore = "Requires all AI API keys"]
#[tokio::test]
async fn bulk_message_propagation() {
    dotenv().ok();
    let client = AIClient::new().unwrap();
    let responses = client
        .completions(
            [
                ClientKind::Anthropic,
                ClientKind::Google,
                ClientKind::Meta,
                ClientKind::Mistral,
                ClientKind::OpenAi,
            ],
            vec![Message {
                content: String::from("Introduce yourself"),
                role: Role::User,
            }],
        )
        .await;
    println!("{responses:#?}")
}
