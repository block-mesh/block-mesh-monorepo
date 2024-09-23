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
pub mod bulk;
pub mod google;
pub mod meta;
pub mod mistral;
pub mod openai;
