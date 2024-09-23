use crate::ai_constants::{
    ANTHROPIC_VAR_NAME, GEMINI_VAR_NAME, LLAMA_VAR_NAME, MISTRAL_VAR_NAME, OPENAI_VAR_NAME,
};
use crate::clients::anthropic::AnthropicClient;
use crate::clients::google::GeminiClient;
use crate::clients::meta::LlamaClient;
use crate::clients::mistral::MistralClient;
use crate::clients::openai::OpenAiClient;
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
}

#[derive(Hash, PartialEq, Eq, Serialize, Deserialize, Clone, Debug)]
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
                    responses.insert(
                        ClientKind::Anthropic,
                        Some(self.anthropic.completion(messages.clone()).await),
                    );
                }
                ClientKind::Google => {
                    responses.insert(
                        ClientKind::Google,
                        Some(self.google.completion(messages.clone()).await),
                    );
                }
                ClientKind::Meta => {
                    responses.insert(
                        ClientKind::Meta,
                        Some(self.meta.completion(messages.clone()).await),
                    );
                }
                ClientKind::Mistral => {
                    responses.insert(
                        ClientKind::Mistral,
                        Some(self.mistral.completion(messages.clone()).await),
                    );
                }
                ClientKind::OpenAi => {
                    responses.insert(
                        ClientKind::OpenAi,
                        Some(self.openai.completion(messages.clone()).await),
                    );
                }
            };
        }
        responses
    }
}

// #[derive(Debug, Default)]
// pub struct AIClientResponses {
//     pub anthropic: Option<anyhow::Result<Message>>,
//     pub google: Option<anyhow::Result<Message>>,
//     pub meta: Option<anyhow::Result<Message>>,
//     pub mistral: Option<anyhow::Result<Message>>,
//     pub openai: Option<anyhow::Result<Message>>,
// }

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
    async fn completion(&self, messages: Vec<Message>) -> anyhow::Result<Message>;
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct Response {
    model: ClientKind,
    question: String,
    answer: String,
}

// #[ignore = "Requires all AI API keys"]
#[tokio::test]
async fn bulk_message_propagation() {
    dotenv().ok();
    let client = AIClient::new().unwrap();

    let root = std::env::var("ROOT").unwrap();
    let now = chrono::offset::Utc::now();
    let dir_name = format!("{}/ai-results/", root);
    let file_name = format!("{}/ai-results/{}.json", root, now.timestamp_millis());
    std::fs::create_dir_all(dir_name).unwrap();

    let mut questsions: Vec<Message> = Vec::new();
    questsions.push(Message {
        content: String::from("Introduce yourself"),
        role: Role::User,
    });
    questsions.push(Message {
        content: String::from("Which company created you"),
        role: Role::User,
    });

    let mut final_responses: Vec<Response> = Vec::new();

    for question in questsions {
        let responses = client
            .completions(
                [
                    ClientKind::Anthropic,
                    ClientKind::Google,
                    // ClientKind::Meta,
                    ClientKind::Mistral,
                    ClientKind::OpenAi,
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
